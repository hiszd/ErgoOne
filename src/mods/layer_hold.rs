use defmt::debug;
use heapless::Vec;

use crate::action;
use crate::actions::CallbackActions;
use crate::key::DEBOUNCE_CYCLES;
use crate::key::HOLD_CYCLES;
use crate::keyscanning::StateType;
use crate::Context;
use crate::ARGS;
use crate::{key::Key, key_codes::KeyCode};

pub trait LayerHold {
    fn lyhnew(s: &str) -> Self
    where
        Self: Sized,
        Self: LayerHold;
    fn lyhtap(&mut self, ctx: Context) -> [Option<KeyCode>; 4];
    fn lyhhold(&mut self, _ctx: Context) -> [Option<KeyCode>; 4];
    fn lyhidle(&mut self, _ctx: Context) -> [Option<KeyCode>; 4];
    fn lyhoff(&mut self, _ctx: Context) -> [Option<KeyCode>; 4];
    fn get_keys(&mut self, ctx: Context) -> [Option<KeyCode>; 4];
    fn lyhscan(&mut self, is_high: bool, ctx: Context) -> [Option<KeyCode>; 4];
}

impl LayerHold for Key {
    fn lyhnew(s: &str) -> Self {
        let sr = s.split(",").map(|s| s.trim()).collect::<Vec<&str, 2>>();
        debug!("sr: {:?}", sr);
        Key {
            cycles: 0,
            raw_state: false,
            cycles_off: 0,
            state: StateType::Off,
            prevstate: StateType::Off,
            keycode: [None; 4],
            previnfo: [false; 6],
            stor: [sr[0].parse().unwrap(), sr[1].parse().unwrap(), 0, 0, 0, 0],
            typ: "LayerHold",
        }
    }
    fn lyhtap(&mut self, _ctx: Context) -> [Option<KeyCode>; 4] {
        if self.prevstate != StateType::Tap {
            debug!("Tap");
            action(
                CallbackActions::SetLayer,
                ARGS::LYR {
                    l: 1,
                },
            );
            self.previnfo[0] = true;
        }
        [None; 4]
    }
    fn lyhhold(&mut self, _ctx: Context) -> [Option<KeyCode>; 4] {
        [None; 4]
    }
    fn lyhidle(&mut self, _ctx: Context) -> [Option<KeyCode>; 4] {
        [None; 4]
    }
    fn lyhoff(&mut self, _ctx: Context) -> [Option<KeyCode>; 4] {
        if self.previnfo[0] {
            debug!("Off: {}, {}", self.previnfo[0], self.prevstate);
            action(
                CallbackActions::SetLayer,
                ARGS::LYR {
                    l: 0,
                },
            );
            self.previnfo[0] = false;
        }
        [None; 4]
    }
    #[doc = " Perform state change as a result of the scan"]
    fn lyhscan(&mut self, is_high: bool, ctx: Context) -> [Option<KeyCode>; 4] {
        //    |                            |
        //    |       Cycle Counters       |
        //    |                            |
        //    |____________________________|

        // set the raw state to the state of the pin
        if is_high {
            // increment cycles while pin is high
            if self.cycles < u16::MAX {
                self.cycles += 1;
            }
            self.cycles_off = 0;
        } else {
            // increment cycles_off while pin is low
            if self.cycles_off < u16::MAX {
                self.cycles_off += 1;
            }
            // reset cycles since pin is low
            self.cycles = 0;
        }
        self.raw_state = is_high;

        //     ____________________________
        //    |                            |
        //    |        State Change        |
        //    |                            |
        //    |____________________________|

        // if we have gotten more cycles in than the debounce_cycles
        if self.cycles >= DEBOUNCE_CYCLES {
            // if the current state is Tap  and we have more cycles than hold_cycles
            if self.state == StateType::Tap && self.cycles >= HOLD_CYCLES {
                self.prevstate = self.state;
                self.state = StateType::Hold;
            } else if self.state == StateType::Off || self.state == StateType::Tap {
                // if the current state is Off
                self.prevstate = self.state;
                self.state = StateType::Tap;
            } else if self.state == StateType::Hold {
                self.prevstate = self.state;
                self.state = StateType::Hold;
            }
            return self.get_keys(ctx);
        } else if self.cycles_off >= 1 {
            self.prevstate = self.state;
            self.state = StateType::Off;
        }
        self.get_keys(ctx)
    }
    fn get_keys(&mut self, ctx: Context) -> [Option<KeyCode>; 4] {
        match self.state {
            StateType::Tap => self.lyhtap(ctx),
            StateType::Hold => self.lyhhold(ctx),
            StateType::Idle => self.lyhidle(ctx),
            StateType::Off => self.lyhoff(ctx),
        }
    }
}

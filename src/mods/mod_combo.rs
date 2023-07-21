use crate::action;
use crate::actions::CallbackActions;
use crate::key::DEBOUNCE_CYCLES;
use crate::key::HOLD_CYCLES;
use crate::keyscanning::StateType;
use crate::Context;
use crate::Operation;
use crate::ARGS;
use crate::{key::Key, key_codes::KeyCode};

pub trait ModCombo {
    fn mcnew(KC1: KeyCode, KC2: KeyCode) -> Self
    where
        Self: Sized,
        Self: ModCombo;
    fn mctap(&mut self, ctx: Context) -> [Option<(KeyCode, Operation)>; 4];
    fn mchold(&mut self, _ctx: Context) -> [Option<(KeyCode, Operation)>; 4];
    fn mcidle(&mut self, _ctx: Context) -> [Option<(KeyCode, Operation)>; 4];
    fn mcoff(&mut self, _ctx: Context) -> [Option<(KeyCode, Operation)>; 4];
    fn get_keys(&mut self, ctx: Context) -> [Option<(KeyCode, Operation)>; 4];
    fn mcscan(&mut self, is_high: bool, ctx: Context) -> [Option<(KeyCode, Operation)>; 4];
}

impl ModCombo for Key {
    fn mcnew(KC1: KeyCode, KC2: KeyCode) -> Self {
        Key {
            cycles: 0,
            raw_state: false,
            cycles_off: 0,
            state: StateType::Off,
            prevstate: StateType::Off,
            keycode: [
                Some((KC1, Operation::SendOn)),
                Some((KC2, Operation::SendOn)),
                None,
                None,
            ],
            previnfo: [false; 6],
            stor: [0; 6],
            typ: "ModCombo",
        }
    }
    fn mctap(&mut self, _ctx: Context) -> [Option<(KeyCode, Operation)>; 4] {
        let [Some(kc0), Some(kc1), None, None] = self.keycode else {
            return [None; 4];
        };
        action(
            CallbackActions::Press,
            ARGS::KS {
                code: kc1.0,
                op: kc1.1,
            },
        );
        action(
            CallbackActions::Press,
            ARGS::KS {
                code: kc0.0,
                op: kc0.1,
            },
        );
        [Some((kc0.0, kc0.1)), Some((kc1.0, kc1.1)), None, None]
    }
    fn mchold(&mut self, _ctx: Context) -> [Option<(KeyCode, Operation)>; 4] {
        let [Some(kc0), Some(kc1), None, None] = self.keycode else {
            return [None; 4];
        };
        [Some((kc0.0, kc0.1)), Some((kc1.0, kc1.1)), None, None]
    }
    fn mcidle(&mut self, _ctx: Context) -> [Option<(KeyCode, Operation)>; 4] {
        [None; 4]
    }
    fn mcoff(&mut self, _ctx: Context) -> [Option<(KeyCode, Operation)>; 4] {
        let [Some(kc0), Some(kc1), None, None] = self.keycode else {
            return [None; 4];
        };
        if self.prevstate != StateType::Off {
            action(
                CallbackActions::Release,
                ARGS::KS {
                    code: kc0.0,
                    op: kc0.1,
                },
            );
            action(
                CallbackActions::Release,
                ARGS::KS {
                    code: kc1.0,
                    op: kc1.1,
                },
            );
        }
        [Some((kc0.0, kc0.1)), Some((kc1.0, kc1.1)), None, None]
    }
    #[doc = " Perform state change as a result of the scan"]
    fn mcscan(&mut self, is_high: bool, ctx: Context) -> [Option<(KeyCode, Operation)>; 4] {
        let [Some(kc0), Some(kc1), None, None] = self.keycode else {
            return [None; 4];
        };
        // println!("{}", is_high);
        // if they KeyCode is empty then don't bother processing
        if kc0.0 == KeyCode::________ && kc1.0 == KeyCode::________ {
            return [None; 4];
        }
        //     ____________________________
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
        // } else if self.cycles_off >= DEBOUNCE_CYCLES.into() {
        } else if self.cycles_off >= 1 {
            self.prevstate = self.state;
            self.state = StateType::Off;
        }
        self.get_keys(ctx)
    }
    fn get_keys(&mut self, ctx: Context) -> [Option<(KeyCode, Operation)>; 4] {
        match self.state {
            StateType::Tap => self.mctap(ctx),
            StateType::Hold => self.mchold(ctx),
            StateType::Idle => self.mcidle(ctx),
            StateType::Off => self.mcoff(ctx),
        }
    }
}

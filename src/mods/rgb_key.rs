use crate::action;
use crate::actions::CallbackActions;
use crate::key::DEBOUNCE_CYCLES;
use crate::key::HOLD_CYCLES;
use crate::keyscanning::StateType;
use crate::Context;
use crate::Operation;
use crate::ARGS;
use crate::{key::Key, key_codes::KeyCode};

pub trait RGBKey {
    fn rknew(r: u8, g: u8, b: u8) -> Self
    where
        Self: Sized,
        Self: RGBKey;
    fn rktap(
        &mut self,
        ctx: Context,
    ) -> [Option<(KeyCode, Operation)>; 4];
    fn rkhold(
        &mut self,
        _ctx: Context,
    ) -> [Option<(KeyCode, Operation)>; 4];
    fn rkidle(
        &mut self,
        _ctx: Context,
    ) -> [Option<(KeyCode, Operation)>; 4];
    fn rkoff(
        &mut self,
        _ctx: Context,
    ) -> [Option<(KeyCode, Operation)>; 4];
    fn get_keys(
        &mut self,
        ctx: Context,
    ) -> [Option<(KeyCode, Operation)>; 4];
    fn rkscan(&mut self, is_high: bool, ctx: Context) -> [Option<(KeyCode, Operation)>; 4];
}

impl RGBKey for Key {
    fn rknew(r: u8, g: u8, b: u8) -> Self {
        Key {
            cycles: 0,
            raw_state: false,
            cycles_off: 0,
            state: StateType::Off,
            prevstate: StateType::Off,
            keycode: [
                Some((KeyCode::EEEEEEEE, Operation::SendOn)),
                Some((KeyCode::EEEEEEEE, Operation::SendOn)),
                None,
                None,
            ],
            previnfo: [false; 6],
            stor: [r, g, b, 0, 0, 0],
            typ: "RGBKey",
        }
    }
    fn rktap(
        &mut self,
        _ctx: Context,
    ) -> [Option<(KeyCode, Operation)>; 4] {
        let [Some(_kc0), Some(kc1), None, None] = self.keycode else {
            return [None; 4];
        };
        action(
            CallbackActions::RGBSet,
            ARGS::RGB {
                r: self.stor[0],
                g: self.stor[1],
                b: self.stor[2],
            },
        );
        [Some((kc1.0, kc1.1)), None, None, None]
    }
    fn rkhold(
        &mut self,
        _ctx: Context,
    ) -> [Option<(KeyCode, Operation)>; 4] {
        let [Some(_kc0), Some(kc1), None, None] = self.keycode else {
            return [None; 4];
        };
        [Some((kc1.0, kc1.1)), None, None, None]
    }
    fn rkidle(
        &mut self,
        _ctx: Context,
    ) -> [Option<(KeyCode, Operation)>; 4] {
        [None; 4]
    }
    fn rkoff(
        &mut self,
        _ctx: Context,
    ) -> [Option<(KeyCode, Operation)>; 4] {
        let [Some(_kc0), Some(kc1), None, None] = self.keycode else {
            return [None; 4];
        };
        [Some((kc1.0, kc1.1)), None, None, None]
    }
    #[doc = " Perform state change as a result of the scan"]
    fn rkscan(
        &mut self,
        is_high: bool,
        ctx: Context,
    ) -> [Option<(KeyCode, Operation)>; 4] {
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
            }
            return self.get_keys(ctx);
        // } else if self.cycles_off >= DEBOUNCE_CYCLES.into() {
        } else if self.cycles_off >= 1 {
            self.prevstate = self.state;
            self.state = StateType::Off;
        }
        self.get_keys(ctx)
    }
    fn get_keys(
        &mut self,
        ctx: Context,
    ) -> [Option<(KeyCode, Operation)>; 4] {
        match self.state {
            StateType::Tap => self.rktap(ctx),
            StateType::Hold => self.rkhold(ctx),
            StateType::Idle => self.rkidle(ctx),
            StateType::Off => self.rkoff(ctx),
        }
    }
}

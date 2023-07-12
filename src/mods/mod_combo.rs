use defmt::error;

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
    fn mctap(
        &mut self,
        ctx: Context,
        action: fn(CallbackActions, ARGS),
    ) -> [(KeyCode, Operation); 2];
    fn mchold(
        &mut self,
        _ctx: Context,
        action: fn(CallbackActions, ARGS),
    ) -> [(KeyCode, Operation); 2];
    fn mcidle(
        &mut self,
        _ctx: Context,
        action: fn(CallbackActions, ARGS),
    ) -> [(KeyCode, Operation); 2];
    fn mcoff(
        &mut self,
        _ctx: Context,
        action: fn(CallbackActions, ARGS),
    ) -> [(KeyCode, Operation); 2];
    fn get_keys(
        &mut self,
        ctx: Context,
        action: fn(CallbackActions, ARGS),
    ) -> [(KeyCode, Operation); 2];
    fn mcscan(
        &mut self,
        is_high: bool,
        ctx: Context,
        action: fn(CallbackActions, ARGS),
    ) -> [(KeyCode, Operation); 2];
}

impl ModCombo for Key {
    fn mcnew(KC1: KeyCode, KC2: KeyCode) -> Self {
        Key {
            cycles: 0,
            raw_state: false,
            cycles_off: 0,
            state: StateType::Off,
            prevstate: StateType::Off,
            keycode: [(KC1, Operation::SendOn), (KC2, Operation::SendOn)],
            previnfo: [false; 6],
            stor: [0; 6],
            typ: "ModCombo",
        }
    }
    fn mctap(
        &mut self,
        _ctx: Context,
        action: fn(CallbackActions, ARGS),
    ) -> [(KeyCode, Operation); 2] {
        action(
            CallbackActions::mPush,
            ARGS::KS {
                code: Some(self.keycode[1].0),
                op: Some(self.keycode[1].1),
            },
        );
        action(
            CallbackActions::iPush,
            ARGS::KS {
                code: Some(self.keycode[0].0),
                op: Some(self.keycode[0].1),
            },
        );
        [
            (self.keycode[0].0, self.keycode[0].1),
            (self.keycode[1].0, self.keycode[1].1),
        ]
    }
    fn mchold(
        &mut self,
        _ctx: Context,
        _action: fn(CallbackActions, ARGS),
    ) -> [(KeyCode, Operation); 2] {
        [
            (self.keycode[0].0, self.keycode[0].1),
            (self.keycode[1].0, self.keycode[1].1),
        ]
    }
    fn mcidle(
        &mut self,
        _ctx: Context,
        _action: fn(CallbackActions, ARGS),
    ) -> [(KeyCode, Operation); 2] {
        [(KeyCode::________, Operation::SendOn); 2]
    }
    fn mcoff(
        &mut self,
        _ctx: Context,
        action: fn(CallbackActions, ARGS),
    ) -> [(KeyCode, Operation); 2] {
        if self.prevstate != StateType::Off {
            action(
                CallbackActions::iPull,
                ARGS::KS {
                    code: Some(self.keycode[0].0),
                    op: Some(self.keycode[0].1),
                },
            );
            action(
                CallbackActions::mPull,
                ARGS::KS {
                    code: Some(self.keycode[1].0),
                    op: Some(self.keycode[1].1),
                },
            );
        }
        [
            (self.keycode[0].0, self.keycode[0].1),
            (self.keycode[1].0, self.keycode[1].1),
        ]
    }
    #[doc = " Perform state change as a result of the scan"]
    fn mcscan(
        &mut self,
        is_high: bool,
        ctx: Context,
        action: fn(CallbackActions, ARGS),
    ) -> [(KeyCode, Operation); 2] {
        // println!("{}", is_high);
        const DEFCODE: [(KeyCode, Operation); 2] = [
            (KeyCode::________, Operation::SendOn),
            (KeyCode::________, Operation::SendOn),
        ];
        // if they KeyCode is empty then don't bother processing
        if self.keycode[0].0 == KeyCode::________ && self.keycode[1].0 == KeyCode::________ {
            return DEFCODE;
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
            return self.get_keys(ctx, action);
        // } else if self.cycles_off >= DEBOUNCE_CYCLES.into() {
        } else if self.cycles_off >= 1 {
            self.prevstate = self.state;
            self.state = StateType::Off;
        }
        self.get_keys(ctx, action)
    }
    fn get_keys(
        &mut self,
        ctx: Context,
        action: fn(CallbackActions, ARGS),
    ) -> [(KeyCode, Operation); 2] {
        match self.state {
            StateType::Tap => self.mctap(ctx, action),
            StateType::Hold => self.mchold(ctx, action),
            StateType::Idle => self.mcidle(ctx, action),
            StateType::Off => self.mcoff(ctx, action),
        }
    }
}

use defmt::error;

use crate::ARGS;
use crate::actions::CallbackActions;
use crate::key::DEBOUNCE_CYCLES;
use crate::key::HOLD_CYCLES;
use crate::keyscanning::StateType;
use crate::Operation;
use crate::{key::Key, key_codes::KeyCode};
use crate::Context;

pub trait ModTap {
    fn mtnew(KC1: KeyCode, KC2: KeyCode) -> Self
    where
        Self: Sized,
        Self: ModTap;
    fn mttap(
        &mut self,
        ctx: Context,
        action: fn(CallbackActions, ARGS),
    ) -> [(KeyCode, Operation); 2];
    fn mthold(
        &mut self,
        _ctx: Context,
        action: fn(CallbackActions, ARGS),
    ) -> [(KeyCode, Operation); 2];
    fn mtidle(
        &mut self,
        _ctx: Context,
        action: fn(CallbackActions, ARGS),
    ) -> [(KeyCode, Operation); 2];
    fn mtoff(
        &mut self,
        _ctx: Context,
        action: fn(CallbackActions, ARGS),
    ) -> [(KeyCode, Operation); 2];
    fn get_keys(
        &mut self,
        ctx: Context,
        action: fn(CallbackActions, ARGS),
    ) -> [(KeyCode, Operation); 2];
    fn mtscan(
        &mut self,
        is_high: bool,
        ctx: Context,
        action: fn(CallbackActions, ARGS),
    ) -> [(KeyCode, Operation); 2];
    fn exist_next(&self, ms: [Option<KeyCode>; 6], ks: [Option<KeyCode>; 6], key: KeyCode) -> bool;
}

impl ModTap for Key {
    fn mtnew(KC1: KeyCode, KC2: KeyCode) -> Self {
        Key {
            cycles: 0,
            raw_state: false,
            cycles_off: 0,
            state: StateType::Off,
            prevstate: StateType::Off,
            keycode: [
                (KC1, Operation::SendOff),
                (KC2, Operation::SendOn),
            ],
            previnfo: [false; 6],
            stor: [0; 6],
            typ: "ModTap",
        }
    }
    // when state becomes tap enqueue modifier
    // when state becomes hold never queue key
    fn mttap(
        &mut self,
        ctx: Context,
        action: fn(CallbackActions, ARGS),
    ) -> [(KeyCode, Operation); 2] {
        let combo: bool;
        if self.keycode[1].0.is_modifier() {
            combo = self.exist_next(ctx.modifiers, ctx.key_queue, self.keycode[1].0);
        } else {
            error!("{} is not a modifier", self.keycode[1].0);
            return [(KeyCode::________, Operation::SendOn); 2];
        }

        self.previnfo[0] = combo;
        match self.keycode[1].0.is_modifier() {
            true => {
                action(CallbackActions::mPush, ARGS::KS { code: Some(self.keycode[1].0), op: Some(self.keycode[1].1)});
            }
            false => error!("{} is not a modifier", self.keycode[1].0),
        }
        [
            (self.keycode[1].0, self.keycode[1].1),
            (KeyCode::________, Operation::SendOn),
        ]
    }
    fn mthold(
        &mut self,
        _ctx: Context,
        action: fn(CallbackActions, ARGS),
    ) -> [(KeyCode, Operation); 2] {
        self.previnfo[0] = true;
        match self.keycode[1].0.is_modifier() {
            true => {
                action(CallbackActions::mPush, ARGS::KS { code: Some(self.keycode[1].0), op: Some(self.keycode[1].1)});
            }
            false => error!("{} is not a modifier", self.keycode[1].0),
        }
        [
            (self.keycode[1].0, self.keycode[1].1),
            (KeyCode::________, Operation::SendOn),
        ]
    }
    fn mtidle(
        &mut self,
        _ctx: Context,
        _action: fn(CallbackActions, ARGS),
    ) -> [(KeyCode, Operation); 2] {
        [(KeyCode::________, Operation::SendOn); 2]
    }
    // when state goes from tap>off and another key was never pressed enqueue key and pull modifier
    // when state goes from tap>off and another key was pressed never queue key and pull modifier
    // when state goed from hold>off never queue key, but pull modifier
    fn mtoff(
        &mut self,
        _ctx: Context,
        action: fn(CallbackActions, ARGS),
    ) -> [(KeyCode, Operation); 2] {
        match self.prevstate {
            StateType::Tap => {
                if !self.previnfo[0] {
                    match self.keycode[0].0.is_modifier() {
                        true => error!("{} is a modifier, but shouldn't be", self.keycode[0].0),
                        false => {
                            action(CallbackActions::iPush, ARGS::KS { code: Some(self.keycode[0].0), op: Some(self.keycode[0].1)});
                            action(CallbackActions::mPull, ARGS::KS { code: Some(self.keycode[1].0), op: Some(self.keycode[1].1)});
                        }
                    }
                    return [
                        (self.keycode[0].0, self.keycode[0].1),
                        (KeyCode::________, Operation::SendOn),
                    ];
                } else {
                    return [(KeyCode::________, Operation::SendOn); 2];
                }
            }
            StateType::Hold => {
                match self.keycode[0].0.is_modifier() {
                    true => error!("{} is a modifier, but shouldn't be", self.keycode[0].0),
                    false => {
                        action(CallbackActions::mPull, ARGS::KS { code: Some(self.keycode[1].0), op: Some(self.keycode[1].1)});
                    }
                }
                return [
                    (self.keycode[1].0, self.keycode[1].1),
                    (KeyCode::________, Operation::SendOn),
                ];
            }
            StateType::Off => {
                return [(KeyCode::________, Operation::SendOn); 2];
            }
            _ => {
                return [(KeyCode::________, Operation::SendOn); 2];
            }
        }
    }

    /// check to see if another key exists in the queue after the current one
    fn exist_next(&self, ms: [Option<KeyCode>; 6], ks: [Option<KeyCode>; 6], key: KeyCode) -> bool {
        let mut rtrn1 = false;
        let mut rtrn2 = false;
        let ind1: Option<usize> = ks.iter().position(|k| k.is_some() && k.unwrap() == key);
        if ind1.is_some() {
            for i in ind1.unwrap() + 1..ks.len() {
                if ks[i].is_some() {
                    rtrn1 = true;
                }
            }
        }
        let ind2: Option<usize> = ms.iter().position(|k| k.is_some() && k.unwrap() == key);
        if ind2.is_some() {
            for i in ind2.unwrap() + 1..ms.len() {
                if ms[i].is_some() {
                    rtrn2 = true;
                }
            }
        }
        rtrn1 || rtrn2
    }
    #[doc = " Perform state change as a result of the scan"]
    fn mtscan(
        &mut self,
        is_high: bool,
        ctx: Context,
        action: fn(CallbackActions, ARGS),
    ) -> [(KeyCode, Operation); 2] {
        const DEFCODE: [(KeyCode, Operation); 2] = [
            (KeyCode::________, Operation::SendOn),
            (KeyCode::________, Operation::SendOn),
        ];
        if self.keycode[0].0 == KeyCode::________ && self.keycode[1].0 == KeyCode::________ {
            return DEFCODE;
        }
        if is_high {
            if self.cycles < u16::MAX {
                self.cycles += 1;
            }
            self.cycles_off = 0;
        } else {
            if self.cycles_off < u16::MAX {
                self.cycles_off += 1;
            }
            self.cycles = 0;
        }
        self.raw_state = is_high;
        if self.cycles >= DEBOUNCE_CYCLES {
            if self.state == StateType::Tap && self.cycles >= HOLD_CYCLES {
                self.prevstate = self.state;
                self.state = StateType::Hold;
            } else if self.state == StateType::Off || self.state == StateType::Tap {
                self.prevstate = self.state;
                self.state = StateType::Tap;
            }
            return self.get_keys(ctx, action);
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
            StateType::Tap => self.mttap(ctx, action),
            StateType::Hold => self.mthold(ctx, action),
            StateType::Idle => self.mtidle(ctx, action),
            StateType::Off => self.mtoff(ctx, action),
        }
    }
}

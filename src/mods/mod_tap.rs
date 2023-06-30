use defmt::error;
use defmt::warn;

use crate::key::DEBOUNCE_CYCLES;
use crate::key::HOLD_CYCLES;
use crate::keyscanning::StateType;
use crate::Operation;
use crate::{key::Key, key_codes::KeyCode};
use crate::{Context, KeyImpl};

pub trait ModTap {
    fn new(KC1: KeyCode, KC2: Option<KeyCode>) -> Self
    where
        Self: Sized;
    fn tap(
        &mut self,
        _ctx: Context,
        action: fn(&str, (Option<KeyCode>, Option<Operation>)),
    ) -> [(KeyCode, Operation); 2];
    fn hold(
        &mut self,
        _ctx: Context,
        action: fn(&str, (Option<KeyCode>, Option<Operation>)),
    ) -> [(KeyCode, Operation); 2];
    fn idle(
        &mut self,
        _ctx: Context,
        action: fn(&str, (Option<KeyCode>, Option<Operation>)),
    ) -> [(KeyCode, Operation); 2];
    fn off(
        &mut self,
        _ctx: Context,
        action: fn(&str, (Option<KeyCode>, Option<Operation>)),
    ) -> [(KeyCode, Operation); 2];
    fn get_keys(
        &mut self,
        ctx: Context,
        action: fn(&str, (Option<KeyCode>, Option<Operation>)),
    ) -> [(KeyCode, Operation); 2];
    fn scan(
        &mut self,
        is_high: bool,
        ctx: Context,
        action: fn(&str, (Option<KeyCode>, Option<Operation>)),
    ) -> [(KeyCode, Operation); 2];
    fn exist_next(&self, ks: [Option<KeyCode>; 6], key: Option<KeyCode>) -> bool;
}

impl ModTap for Key {
    fn new(KC1: KeyCode, KC2: Option<KeyCode>) -> Self {
        Key {
            cycles: 0,
            raw_state: false,
            cycles_off: 0,
            state: StateType::Off,
            prevstate: StateType::Off,
            keycode: [
                (KC1, Operation::SendOff),
                (KC2.unwrap_or(KeyCode::________), Operation::SendOn),
            ],
            previnfo: [false; 6],
        }
    }
    // when state becomes tap enqueue modifier
    // when state becomes hold never queue key
    // when state goes from tap>off and another key was never pressed enqueue key
    // when state goes from tap>off ant another key was pressed never queue key
    fn tap(
        &mut self,
        ctx: Context,
        action: fn(&str, (Option<KeyCode>, Option<Operation>)),
    ) -> [(KeyCode, Operation); 2] {
        let combo: bool;
        if self.keycode[0].0.is_modifier() {
            combo = self.exist_next(ctx.key_queue, None);
        } else {
            combo = self.exist_next(ctx.key_queue, Some(self.keycode[0].0));
        }


        if self.state != self.prevstate {
            action("mpush", (Some(self.keycode[1].0), Some(self.keycode[1].1)));
        }


        self.previnfo[0] = combo;
        if combo {
            match self.keycode[1].0.is_modifier() {
                true => action("mpush", (Some(self.keycode[1].0), Some(self.keycode[1].1))),
                false => action("ipush", (Some(self.keycode[1].0), Some(self.keycode[1].1))),
            }
            [
                (self.keycode[1].0, self.keycode[1].1),
                (KeyCode::________, Operation::SendOn),
            ]
        } else {
            match self.keycode[0].0.is_modifier() {
                true => action("mpush", (Some(self.keycode[0].0), Some(self.keycode[0].1))),
                false => action("ipush", (Some(self.keycode[0].0), Some(self.keycode[0].1))),
            }
            [
                (self.keycode[0].0, self.keycode[1].1),
                (KeyCode::________, Operation::SendOn),
            ]
        }
    }
    fn hold(
        &mut self,
        _ctx: Context,
        action: fn(&str, (Option<KeyCode>, Option<Operation>)),
    ) -> [(KeyCode, Operation); 2] {
        self.previnfo[0] = true;
        match self.keycode[1].0.is_modifier() {
            true => action("mpush", (Some(self.keycode[1].0), Some(self.keycode[1].1))),
            false => action("ipush", (Some(self.keycode[1].0), Some(self.keycode[1].1))),
        }
        [
            (self.keycode[1].0, self.keycode[1].1),
            (KeyCode::________, Operation::SendOn),
        ]
    }
    fn idle(
        &mut self,
        _ctx: Context,
        _action: fn(&str, (Option<KeyCode>, Option<Operation>)),
    ) -> [(KeyCode, Operation); 2] {
        [
            (KeyCode::________, Operation::SendOn),
            (KeyCode::________, Operation::SendOn),
        ]
    }
    fn off(
        &mut self,
        _ctx: Context,
        action: fn(&str, (Option<KeyCode>, Option<Operation>)),
    ) -> [(KeyCode, Operation); 2] {
        let rtrnnum: u8;
        if self.prevstate == StateType::Tap {
            if self.previnfo[0] {
                self.previnfo[0] = false;
                rtrnnum = 1;
            } else {
                rtrnnum = 2;
            }
        } else if self.prevstate == StateType::Hold || self.previnfo[0] {
            if self.previnfo[0] {
                self.previnfo[0] = false;
                rtrnnum = 2;
            } else {
                rtrnnum = 1;
            }
        } else if self.prevstate == StateType::Off {
            rtrnnum = 0;
        } else {
            error!("Invalid state type: {}", self.prevstate);
            rtrnnum = 0;
        }

        if rtrnnum == 1 {
            match self.keycode[0].0.is_modifier() {
                true => action("mpush", (Some(self.keycode[0].0), Some(self.keycode[0].1))),
                false => action("ipush", (Some(self.keycode[0].0), Some(self.keycode[0].1))),
            }
            [
                (self.keycode[0].0, self.keycode[0].1),
                (KeyCode::________, Operation::SendOn),
            ]
        } else if rtrnnum == 2 {
            match self.keycode[1].0.is_modifier() {
                true => action("mpush", (Some(self.keycode[1].0), Some(self.keycode[1].1))),
                false => action("ipush", (Some(self.keycode[1].0), Some(self.keycode[1].1))),
            }
            [
                (self.keycode[1].0, self.keycode[1].1),
                (KeyCode::________, Operation::SendOn),
            ]
        } else {
            [
                (KeyCode::________, Operation::SendOn),
                (KeyCode::________, Operation::SendOn),
            ]
        }
    }

    /// check to see if another key exists in the queue after the current one
    fn exist_next(&self, ks: [Option<KeyCode>; 6], key: Option<KeyCode>) -> bool {
        if key.is_some() {
            let ind: usize = ks
                .iter()
                .position(|k| k.is_some() && k.unwrap() == key.unwrap())
                .unwrap();
            for i in ind + 1..ks.len() {
                if ks[i].is_some() {
                    return true;
                }
            }
            false
        } else {
            ks.iter().any(|k| k.is_some())
        }
    }
    KeyImpl!(ModTap);
}

#[allow(unused_macros)]
#[macro_export]
macro_rules! mt {
    ($code1:expr, $code2:expr) => {
        ModTap::new($code1, Some($code2))
    };
}

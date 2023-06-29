use defmt::error;

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
    fn tap(&mut self, _ctx: Context, inp_call: (fn((KeyCode, Operation)), fn(KeyCode)), mod_call: (fn(KeyCode), fn(KeyCode))) -> [(KeyCode, Operation); 2];
    fn hold(&mut self, _ctx: Context, inp_call: (fn((KeyCode, Operation)), fn(KeyCode)), mod_call: (fn(KeyCode), fn(KeyCode))) -> [(KeyCode, Operation); 2];
    fn idle(&mut self, _ctx: Context, inp_call: (fn((KeyCode, Operation)), fn(KeyCode)), mod_call: (fn(KeyCode), fn(KeyCode))) -> [(KeyCode, Operation); 2];
    fn off(&mut self, _ctx: Context, inp_call: (fn((KeyCode, Operation)), fn(KeyCode)), mod_call: (fn(KeyCode), fn(KeyCode))) -> [(KeyCode, Operation); 2];
    fn get_keys(&mut self, ctx: Context, inp_call: (fn((KeyCode, Operation)), fn(KeyCode)), mod_call: (fn(KeyCode), fn(KeyCode))) -> [(KeyCode, Operation); 2];
    fn scan(&mut self, is_high: bool, ctx: Context, inp_call: (fn((KeyCode, Operation)), fn(KeyCode)), mod_call: (fn(KeyCode), fn(KeyCode))) -> [(KeyCode, Operation); 2];
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
    fn tap(&mut self, ctx: Context, inp_call: (fn((KeyCode, Operation)), fn(KeyCode)), mod_call: (fn(KeyCode), fn(KeyCode))) -> [(KeyCode, Operation); 2] {
        let mut combo: bool;
        if self.keycode[0].0.is_modifier() {
            combo = self.exist_next(ctx.key_queue, None);
        } else {
            combo = self.exist_next(ctx.key_queue, Some(self.keycode[0].0));
        }
        self.previnfo[0] = combo;
        if combo {
            // **************************************
            // this is the new way to do this!
            // update all other actions
            // **************************************
            if self.keycode[0].0.is_modifier() {
                mod_call.0(self.keycode[0].0);
            }
            [
                (self.keycode[1].0, self.keycode[1].1),
                (KeyCode::________, Operation::SendOn),
            ]
        } else {
            [
                (self.keycode[0].0, self.keycode[1].1),
                (KeyCode::________, Operation::SendOn),
            ]
        }
    }
    fn hold(&mut self, _ctx: Context, inp_call: (fn((KeyCode, Operation)), fn(KeyCode)), mod_call: (fn(KeyCode), fn(KeyCode))) -> [(KeyCode, Operation); 2] {
        self.previnfo[0] = true;
        [
            (self.keycode[1].0, self.keycode[1].1),
            (KeyCode::________, Operation::SendOn),
        ]
    }
    fn idle(&mut self, _ctx: Context, inp_call: (fn((KeyCode, Operation)), fn(KeyCode)), mod_call: (fn(KeyCode), fn(KeyCode))) -> [(KeyCode, Operation); 2] {
        [
            (KeyCode::________, Operation::SendOn),
            (KeyCode::________, Operation::SendOn),
        ]
    }
    fn off(&mut self, _ctx: Context, inp_call: (fn((KeyCode, Operation)), fn(KeyCode)), mod_call: (fn(KeyCode), fn(KeyCode))) -> [(KeyCode, Operation); 2] {
        if self.prevstate == StateType::Tap {
            if self.previnfo[0] {
                self.previnfo[0] = false;
                [
                    (self.keycode[0].0, self.keycode[0].1),
                    (KeyCode::________, Operation::SendOn),
                ]
            } else {
                [
                    (self.keycode[1].0, self.keycode[1].1),
                    (KeyCode::________, Operation::SendOn),
                ]
            }
        } else if self.prevstate == StateType::Hold || self.previnfo[0] {
            if self.previnfo[0] {
                self.previnfo[0] = false;
                [
                    (self.keycode[1].0, self.keycode[1].1),
                    (KeyCode::________, Operation::SendOn),
                ]
            } else {
                [
                    (self.keycode[0].0, self.keycode[0].1),
                    (KeyCode::________, Operation::SendOn),
                ]
            }
        } else if self.prevstate == StateType::Off {
            [
                (KeyCode::________, Operation::SendOn),
                (KeyCode::________, Operation::SendOn),
            ]
        } else {
            error!("Invalid state type: {}", self.prevstate);
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

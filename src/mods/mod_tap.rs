use defmt::error;

use crate::Context;
use crate::keyscanning::StateType;
use crate::{key::Key, key_codes::KeyCode};

pub trait ModTap {
    fn new(KC1: KeyCode, KC2: Option<KeyCode>) -> Self
    where
        Self: Sized;
}

impl ModTap for Key {
    fn new(KC1: KeyCode, KC2: Option<KeyCode>) -> Self {
        Key {
            cycles: 0,
            raw_state: false,
            cycles_off: 0,
            state: StateType::Off,
            prevstate: StateType::Off,
            keycode: [KC1, KC2.unwrap_or(KeyCode::________)],
            previnfo: [false; 6],
            tap: |keycodes: [KeyCode; 2], ctx: Context| {
                let mut combo: bool = false;
                if keycodes[0].is_modifier() {
                    ctx.key_queue.unwrap().get_keys().iter().for_each(|k| {
                        if k.is_some() {
                            combo = true;
                        }
                    })
                }
                if combo {
                    [keycodes[1], KeyCode::________]
                } else {
                    [keycodes[0], KeyCode::________]
                }
            },
            hold: |keycodes: [KeyCode; 2], ctx: Context| [keycodes[1], KeyCode::________],
            idle: |_keycodes: [KeyCode; 2], ctx: Context| [KeyCode::________, KeyCode::________],
            off: |keycodes: [KeyCode; 2], prevstate: StateType, ctx: Context| {
                let mut combo: bool = false;
                if keycodes[0].is_modifier() {
                    ctx.key_queue.unwrap().get_keys().iter().for_each(|k| {
                        if k.is_some() {
                            combo = true;
                        }
                    })
                }
                if prevstate == StateType::Tap && !combo {
                    [keycodes[0], KeyCode::________]
                } else if prevstate == StateType::Hold || combo{
                    [keycodes[1], KeyCode::________]
                } else if prevstate == StateType::Off {
                    [KeyCode::________, KeyCode::________]
                } else {
                    error!("Invalid state type: {}", prevstate);
                    [KeyCode::________, KeyCode::________]
                }
            },
        }
    }
}

#[allow(unused_macros)]
#[macro_export]
macro_rules! mt {
    ($code1:expr, $code2:expr) => {
        ModTap::new($code1, Some($code2))
    };
}

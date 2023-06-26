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
            tap: |keycodes: [KeyCode; 2]| {
                keycodes
            },
            hold: |keycodes: [KeyCode; 2]| {
                keycodes
            },
            idle: |keycodes: [KeyCode; 2]| {
                keycodes
            },
            off: |keycodes: [KeyCode; 2]| {
                keycodes
            },
        }
    }
}

#[allow(unused_macros)]
macro_rules! mt {
    ($code1:expr, $code2:expr) => {
        MTKey::new($code1, $code2)
    };
}

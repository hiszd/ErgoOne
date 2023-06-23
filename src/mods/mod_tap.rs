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
                let curcode = keycodes[0];
                let mut modi: u8 = 0;
                if let Some(bitmask) = curcode.modifier_bitmask() {
                    modi |= bitmask;
                    ([KeyCode::________, KeyCode::________], modi)
                } else {
                    ([keycodes[0], KeyCode::________], modi)
                }
            },
            hold: |keycodes: [KeyCode; 2]| {
                let curcode = keycodes[0];
                let mut modi: u8 = 0;
                if let Some(bitmask) = curcode.modifier_bitmask() {
                    modi |= bitmask;
                    ([KeyCode::________, KeyCode::________], modi)
                } else {
                    ([keycodes[0], KeyCode::________], modi)
                }
            },
            idle: |keycodes: [KeyCode; 2]| {
                let curcode = keycodes[0];
                let mut modi: u8 = 0;
                if let Some(bitmask) = curcode.modifier_bitmask() {
                    modi |= bitmask;
                    ([KeyCode::________, KeyCode::________], modi)
                } else {
                    ([keycodes[0], KeyCode::________], modi)
                }
            },
            off: |keycodes: [KeyCode; 2]| {
                let curcode = keycodes[0];
                let mut modi: u8 = 0;
                if let Some(bitmask) = curcode.modifier_bitmask() {
                    modi |= bitmask;
                    ([KeyCode::________, KeyCode::________], modi)
                } else {
                    ([keycodes[0], KeyCode::________], modi)
                }
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

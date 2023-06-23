use crate::key::Key;
use crate::{key::KeyBase, key_codes::KeyCode};

pub trait ModTap {
    fn new_ModTap(KC1: KeyCode, KC2: KeyCode) -> Self;
    fn tap(&self) -> ([KeyCode; 2], u8);
    fn hold(&self) -> ([KeyCode; 2], u8);
    fn idle(&self) -> ([KeyCode; 2], u8);
}

impl ModTap for Key {
    fn new_ModTap(KC1: KeyCode, KC2: KeyCode) -> Self {
        Key {
            keystate: KeyBase::new(KC1, KC2),
        }
    }
    fn tap(&self) -> ([KeyCode; 2], u8) {
        let curcode = self.keystate.keycode[0];
        let mut modi: u8 = 0;
        if let Some(bitmask) = curcode.modifier_bitmask() {
            modi |= bitmask;
            ([KeyCode::________, KeyCode::________], modi)
        } else {
            ([self.keystate.keycode[0], KeyCode::________], modi)
        }
    }
    fn hold(&self) -> ([KeyCode; 2], u8) {
        let curcode = self.keystate.keycode[0];
        let mut modi: u8 = 0;
        if let Some(bitmask) = curcode.modifier_bitmask() {
            modi |= bitmask;
            ([KeyCode::________, KeyCode::________], modi)
        } else {
            ([self.keystate.keycode[0], KeyCode::________], modi)
        }
    }
    fn idle(&self) -> ([KeyCode; 2], u8) {
        let curcode = self.keystate.keycode[0];
        let mut modi: u8 = 0;
        if let Some(bitmask) = curcode.modifier_bitmask() {
            modi |= bitmask;
            ([KeyCode::________, KeyCode::________], modi)
        } else {
            ([self.keystate.keycode[0], KeyCode::________], modi)
        }
    }
}

#[allow(unused_macros)]
macro_rules! mt {
    ($code1:expr, $code2:expr) => {
        MTKey::new($code1, $code2)
    };
}

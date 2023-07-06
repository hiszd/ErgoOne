use defmt::error;

use crate::key::DEBOUNCE_CYCLES;
use crate::key::HOLD_CYCLES;
use crate::keyscanning::StateType;
use crate::Operation;
use crate::{key::Key, key_codes::KeyCode};
use crate::Context;

pub trait RGBKey {
    fn rknew(KC1: KeyCode, KC2: KeyCode) -> Self
    where
        Self: Sized,
        Self: RGBKey;
    fn rktap(
        &mut self,
        ctx: Context,
        action: fn(&str, (Option<KeyCode>, Option<Operation>)),
    ) -> [(KeyCode, Operation); 2];
    fn rkhold(
        &mut self,
        _ctx: Context,
        action: fn(&str, (Option<KeyCode>, Option<Operation>)),
    ) -> [(KeyCode, Operation); 2];
    fn rkidle(
        &mut self,
        _ctx: Context,
        action: fn(&str, (Option<KeyCode>, Option<Operation>)),
    ) -> [(KeyCode, Operation); 2];
    fn rkoff(
        &mut self,
        _ctx: Context,
        action: fn(&str, (Option<KeyCode>, Option<Operation>)),
    ) -> [(KeyCode, Operation); 2];
    fn get_keys(
        &mut self,
        ctx: Context,
        action: fn(&str, (Option<KeyCode>, Option<Operation>)),
    ) -> [(KeyCode, Operation); 2];
    fn rkscan(
        &mut self,
        is_high: bool,
        ctx: Context,
        action: fn(&str, (Option<KeyCode>, Option<Operation>)),
    ) -> [(KeyCode, Operation); 2];
}

impl RGBKey for Key {
    fn rknew(KC1: KeyCode, KC2: KeyCode) -> Self {
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
            typ: "RGBKey",
        }
    }
    // when state becomes tap enqueue modifier
    // when state becomes hold never queue key
    fn rktap(
        &mut self,
        ctx: Context,
        action: fn(&str, (Option<KeyCode>, Option<Operation>)),
    ) -> [(KeyCode, Operation); 2] {
        [
            (self.keycode[1].0, self.keycode[1].1),
            (KeyCode::________, Operation::SendOn),
        ]
    }
    fn rkhold(
        &mut self,
        _ctx: Context,
        action: fn(&str, (Option<KeyCode>, Option<Operation>)),
    ) -> [(KeyCode, Operation); 2] {
        [
            (self.keycode[1].0, self.keycode[1].1),
            (KeyCode::________, Operation::SendOn),
        ]
    }
    fn rkidle(
        &mut self,
        _ctx: Context,
        _action: fn(&str, (Option<KeyCode>, Option<Operation>)),
    ) -> [(KeyCode, Operation); 2] {
        [(KeyCode::________, Operation::SendOn); 2]
    }
    // when state goes from tap>off and another key was never pressed enqueue key and pull modifier
    // when state goes from tap>off and another key was pressed never queue key and pull modifier
    // when state goed from hold>off never queue key, but pull modifier
    fn rkoff(
        &mut self,
        _ctx: Context,
        action: fn(&str, (Option<KeyCode>, Option<Operation>)),
    ) -> [(KeyCode, Operation); 2] {
        [
            (self.keycode[1].0, self.keycode[1].1),
            (KeyCode::________, Operation::SendOn),
        ]
    }
    #[doc = " Perform state change as a result of the scan"]
    fn rkscan(
        &mut self,
        is_high: bool,
        ctx: Context,
        action: fn(&str, (Option<KeyCode>, Option<Operation>)),
    ) -> [(KeyCode, Operation); 2] {
        [
            (self.keycode[1].0, self.keycode[1].1),
            (KeyCode::________, Operation::SendOn),
        ]
    }
    fn get_keys(
        &mut self,
        ctx: Context,
        action: fn(&str, (Option<KeyCode>, Option<Operation>)),
    ) -> [(KeyCode, Operation); 2] {
        match self.state {
            StateType::Tap => self.rktap(ctx, action),
            StateType::Hold => self.rkhold(ctx, action),
            StateType::Idle => self.rkidle(ctx, action),
            StateType::Off => self.rkoff(ctx, action),
        }
    }
}

#[allow(unused_macros)]
#[macro_export]
macro_rules! t {
    ($code1:expr, $code2:expr) => {
        RGBKey::rknew($code1, $code2)
    };
}

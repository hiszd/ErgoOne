#![allow(unused_imports)]
use crate::Operation;
use defmt::export::debug;
use defmt::{info, println};

use crate::{key_codes::KeyCode, keyscanning::StateType};
use crate::{Context, KeyImpl};

pub(crate) const DEBOUNCE_CYCLES: u16 = 3;
pub(crate) const HOLD_CYCLES: u16 = 20;

// TODO impl idle tracking
// const IDLE_CYCLES: u8 = 100;

// #[derive(Copy, Clone, PartialEq, PartialOrd)]
#[derive(Copy, Clone)]
pub struct Key {
    /// The cycles that have passed since the key was pressed(until u16::MAX)
    pub cycles: u16,
    /// The boolean state of the input pin(false = low, true = high)
    pub raw_state: bool,
    /// The cycles that have passed since the key was pressed(until u16::MAX)
    pub cycles_off: u16,
    /// The state that the key currently is
    pub state: StateType,
    /// The state that the key was last time the matrix polled
    pub prevstate: StateType,
    /// Array with a width of 2 where keycode[0] is the normal key and keycode[1] is the function
    /// key
    pub keycode: [(KeyCode, Operation); 2],
    /// Array of booleans for modules to use as a way to store information from the previous poll
    pub previnfo: [bool; 6],
    /// holds a &str of the type of the key
    pub typ: &'static str,
}

pub trait Default {
    fn new(KC1: KeyCode, KC2: Option<KeyCode>) -> Self
    where
        Self: Sized,
        Self: Default;
    fn tap(
        &mut self,
        ctx: Context,
        action: fn(&str, (Option<KeyCode>, Option<Operation>)),
    ) -> [(KeyCode, Operation); 2];
    fn hold(
        &mut self,
        ctx: Context,
        action: fn(&str, (Option<KeyCode>, Option<Operation>)),
    ) -> [(KeyCode, Operation); 2];
    fn idle(
        &self,
        _ctx: Context,
        _action: fn(&str, (Option<KeyCode>, Option<Operation>)),
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
}

impl Default for Key {
    fn new(KC1: KeyCode, KC2: Option<KeyCode>) -> Self {
        Key {
            cycles: 0,
            raw_state: false,
            cycles_off: 0,
            state: StateType::Off,
            prevstate: StateType::Off,
            keycode: [
                (KC1, Operation::SendOn),
                (KC2.unwrap_or(KeyCode::________), Operation::SendOn),
            ],
            previnfo: [false; 6],
            typ: "Default",
        }
    }
    fn tap(
        &mut self,
        _ctx: Context,
        action: fn(&str, (Option<KeyCode>, Option<Operation>)),
    ) -> [(KeyCode, Operation); 2] {
        match self.keycode[0].0.is_modifier() {
            true => action("mpush", (Some(self.keycode[0].0), Some(self.keycode[0].1))),
            false => action("ipush", (Some(self.keycode[0].0), Some(self.keycode[0].1))),
        }
        self.keycode
    }
    fn hold(
        &mut self,
        _ctx: Context,
        action: fn(&str, (Option<KeyCode>, Option<Operation>)),
    ) -> [(KeyCode, Operation); 2] {
        match self.keycode[0].0.is_modifier() {
            true => action("mpush", (Some(self.keycode[0].0), Some(self.keycode[0].1))),
            false => action("ipush", (Some(self.keycode[0].0), Some(self.keycode[0].1))),
        }
        self.keycode
    }
    fn idle(
        &self,
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
        if self.state != self.prevstate {
            match self.keycode[0].0.is_modifier() {
                true => action("mpull", (Some(self.keycode[0].0), Some(self.keycode[0].1))),
                false => action("ipull", (Some(self.keycode[0].0), Some(self.keycode[0].1))),
            }
        }
        self.keycode
    }
    KeyImpl!(Default);
}


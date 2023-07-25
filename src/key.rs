#![allow(unused_imports)]
use crate::action;
use crate::actions::CallbackActions;
use crate::ARGS;
use defmt::export::debug;
use defmt::{info, println};

use crate::{key_codes::KeyCode, keyscanning::StateType};
use crate::{Context, KeyImpl};

pub(crate) const DEBOUNCE_CYCLES: u16 = 3;
pub(crate) const HOLD_CYCLES: u16 = 20;

// TODO impl idle tracking
// const IDLE_CYCLES: u8 = 100;

// #[derive(Copy, Clone, PartialEq, PartialOrd)]
#[derive(Copy, Clone, Debug)]
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
    pub keycode: [Option<KeyCode>; 4],
    /// Array of booleans for modules to use as a way to store information from the previous poll
    pub previnfo: [bool; 6],
    /// Stores information needed by internal functions(e.g. colors for RGB keys)
    pub stor: [u8; 6],
    /// holds a &str of the type of the key
    pub typ: &'static str,
}

pub trait Default {
    fn new(KC1: KeyCode, KC2: Option<KeyCode>) -> Self
    where
        Self: Sized,
        Self: Default;
    fn tap(&mut self, ctx: Context) -> [Option<KeyCode>; 4];
    fn hold(&mut self, ctx: Context) -> [Option<KeyCode>; 4];
    fn idle(&self, _ctx: Context) -> [Option<KeyCode>; 4];
    fn off(&mut self, _ctx: Context) -> [Option<KeyCode>; 4];
    fn get_keys(&mut self, ctx: Context) -> [Option<KeyCode>; 4];
    fn scan(&mut self, is_high: bool, ctx: Context) -> [Option<KeyCode>; 4];
}

impl Default for Key {
    fn new(KC1: KeyCode, _KC2: Option<KeyCode>) -> Self {
        Key {
            cycles: 0,
            raw_state: false,
            cycles_off: 0,
            state: StateType::Off,
            prevstate: StateType::Off,
            keycode: [Some(KC1), None, None, None],
            previnfo: [false; 6],
            stor: [0; 6],
            typ: "Default",
        }
    }
    fn tap(&mut self, _ctx: Context) -> [Option<KeyCode>; 4] {
        if self.keycode[0].is_some() {
            let kc0 = self.keycode[0].unwrap();
            if self.prevstate == StateType::Off {
                action(
                    CallbackActions::Press,
                    ARGS::KS {
                        code: kc0,
                    },
                );
            }
        }
        self.keycode
    }
    fn hold(&mut self, _ctx: Context) -> [Option<KeyCode>; 4] {
        if self.keycode[0].is_some() {
            let kc0 = self.keycode[0].unwrap();
            action(
                CallbackActions::Press,
                ARGS::KS {
                    code: kc0,
                },
            );
            self.keycode
        } else {
            [None; 4]
        }
    }
    fn idle(&self, _ctx: Context) -> [Option<KeyCode>; 4] {
        [None; 4]
    }
    fn off(&mut self, _ctx: Context) -> [Option<KeyCode>; 4] {
        if self.keycode[0].is_some() {
            let kc0 = self.keycode[0].unwrap();
            if self.state != self.prevstate {
                action(
                    CallbackActions::Release,
                    ARGS::KS {
                        code: kc0,
                    },
                );
            }
            self.keycode
        } else {
            return [None; 4];
        }
    }
    KeyImpl!(Default);
}

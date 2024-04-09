#![allow(unused_imports)]
use defmt::export::debug;
use defmt::{info, println};

use crate::actions::CallbackActions;
use crate::Context;
use crate::ARGS;
use crate::{action, modules};
use crate::{key_codes::KeyCode, keyscanning::StateType};
pub(crate) const DEBOUNCE_CYCLES: u16 = 3;
pub(crate) const HOLD_CYCLES: u16 = 20;
// TODO: impl idle tracking
// const IDLE_CYCLES: u8 = 100;

modules!(Default, ModTap);

pub const MOD_STR: &str = "dft";

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
  /// holds a &str for sending
  pub strng: &'static str,
}

impl Key {
  fn get_keys(&mut self, ctx: Context) -> [Option<KeyCode>; 4] {
    match self.state {
      StateType::Tap => self.tap(ctx),
      StateType::Hold => self.hold(ctx),
      StateType::Idle => self.idle(ctx),
      StateType::Off => self.off(ctx),
    }
  }
  pub fn scan(&mut self, is_high: bool, ctx: Context) -> [Option<KeyCode>; 4] {
    // if they KeyCode is empty then don't bother processing
    if self.keycode.iter().fold(true, |acc, s| {
      if s.is_some() {
        return false;
      }
      acc
    }) {
      return [None; 4];
    }
    //     ____________________________
    //    |                            |
    //    |       Cycle Counters       |
    //    |                            |
    //    |____________________________|
    // set the raw state to the state of the pin
    if is_high {
      // increment cycles while pin is high
      if self.cycles < u16::MAX {
        self.cycles += 1;
      }
      self.cycles_off = 0;
    } else {
      // increment cycles_off while pin is low
      if self.cycles_off < u16::MAX {
        self.cycles_off += 1;
      }
      // reset cycles since pin is low
      self.cycles = 0;
    }
    self.raw_state = is_high;
    //     ____________________________
    //    |                            |
    //    |        State Change        |
    //    |                            |
    //    |____________________________|
    // if we have gotten more cycles in than the debounce_cycles
    if self.cycles >= DEBOUNCE_CYCLES {
      if self.state == StateType::Tap && self.cycles >= HOLD_CYCLES {
        self.prevstate = self.state;
        self.state = StateType::Hold;
      } else if self.state == StateType::Off || self.state == StateType::Tap {
        self.prevstate = self.state;
        self.state = StateType::Tap;
      } else if self.state == StateType::Hold {
        self.prevstate = self.state;
        self.state = StateType::Hold;
      }
      return self.get_keys(ctx);
    } else if self.cycles_off >= 1 {
      self.prevstate = self.state;
      self.state = StateType::Off;
    }
    self.get_keys(ctx)
  }
}

pub trait Default {
  fn new(KC1: KeyCode) -> Self
  where
    Self: Sized,
    Self: Default;
  fn tap(&mut self, ctx: Context) -> [Option<KeyCode>; 4];
  fn hold(&mut self, ctx: Context) -> [Option<KeyCode>; 4];
  fn idle(&self, _ctx: Context) -> [Option<KeyCode>; 4];
  fn off(&mut self, _ctx: Context) -> [Option<KeyCode>; 4];
}

impl Default for Key {
  fn new(KC1: KeyCode) -> Self {
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
      strng: "",
    }
  }

  fn tap(&mut self, _ctx: Context) -> [Option<KeyCode>; 4] {
    if self.keycode[0].is_some() {
      let kc0 = self.keycode[0].unwrap();
      if self.prevstate == StateType::Off {
        action(CallbackActions::Press, ARGS::KS { code: kc0 });
      }
    }
    self.keycode
  }

  fn hold(&mut self, _ctx: Context) -> [Option<KeyCode>; 4] {
    if self.keycode[0].is_some() {
      let kc0 = self.keycode[0].unwrap();
      action(CallbackActions::Press, ARGS::KS { code: kc0 });
      self.keycode
    } else {
      [None; 4]
    }
  }

  fn idle(&self, _ctx: Context) -> [Option<KeyCode>; 4] { [None; 4] }

  fn off(&mut self, _ctx: Context) -> [Option<KeyCode>; 4] {
    if self.keycode[0].is_some() {
      let kc0 = self.keycode[0].unwrap();
      if self.state != self.prevstate {
        action(CallbackActions::Release, ARGS::KS { code: kc0 });
      }
      self.keycode
    } else {
      return [None; 4];
    }
  }
}

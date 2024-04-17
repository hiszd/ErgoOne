#![allow(unused_imports)]
use defmt::export::debug;
use defmt::{error, info, println};
use heapless::Vec;

use crate::actions::CallbackActions;
use crate::mods::*;
use crate::Context;
use crate::ARGS;
use crate::{action, ct_idents, modules};
use crate::{key_codes::KeyCode, keyscanning::StateType};
pub(crate) const DEBOUNCE_CYCLES: u16 = 3;
pub(crate) const HOLD_CYCLES: u16 = 20;
// TODO: impl idle tracking
// const IDLE_CYCLES: u8 = 100;

#[allow(dead_code)]
#[derive(Copy, Clone, Debug)]
pub enum Modules {
  Default,
  ModTap,
  ModCombo,
  TapCom,
  TapStr,
  RGBKey,
  SendString,
  SendHIDRaw,
  LayerHold,
  Transparent,
}

impl Modules {
  #[allow(unused)]
  pub fn to_str(&self) -> &'static str {
    match self {
      Modules::Default => "dft",
      Modules::ModTap => "mdt",
      Modules::ModCombo => "mdc",
      Modules::TapCom => "tpc",
      Modules::TapStr => "tps",
      Modules::RGBKey => "rgk",
      Modules::SendString => "sst",
      Modules::SendHIDRaw => "shr",
      Modules::LayerHold => "lyh",
      Modules::Transparent => "transparent",
    }
  }
  pub fn try_from_str(s: &str) -> Result<Self, ()> {
    match s {
      "dft" => Ok(Modules::Default),
      "mdt" => Ok(Modules::ModTap),
      "mdc" => Ok(Modules::ModCombo),
      "tpc" => Ok(Modules::TapCom),
      "tps" => Ok(Modules::TapStr),
      "rgk" => Ok(Modules::RGBKey),
      "sst" => Ok(Modules::SendString),
      "shr" => Ok(Modules::SendHIDRaw),
      "lyh" => Ok(Modules::LayerHold),
      "transparent" => Ok(Modules::Transparent),
      _ => Err(()),
    }
  }
}

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
  pub typ: Modules,
  /// holds a &str for sending
  pub strng: &'static str,
}

impl Key {
  fn get_keys(&mut self, ctx: Context) -> [Option<KeyCode>; 4] {
    match self.typ {
      Modules::Default => match self.state {
        StateType::Tap => <Key as Default>::tap(self),
        StateType::Hold => <Key as Default>::hold(self),
        StateType::Idle => <Key as Default>::idle(self),
        StateType::Off => <Key as Default>::off(self),
      },
      Modules::ModTap => match self.state {
        StateType::Tap => <Key as mod_tap::ModTap>::tap(self, ctx),
        StateType::Hold => <Key as mod_tap::ModTap>::hold(self, ctx),
        StateType::Idle => <Key as mod_tap::ModTap>::idle(self, ctx),
        StateType::Off => <Key as mod_tap::ModTap>::off(self, ctx),
      },
      Modules::TapCom => match self.state {
        StateType::Tap => <Key as mod_tapcom::TapCom>::tap(self, ctx),
        StateType::Hold => <Key as mod_tapcom::TapCom>::hold(self, ctx),
        StateType::Idle => <Key as mod_tapcom::TapCom>::idle(self, ctx),
        StateType::Off => <Key as mod_tapcom::TapCom>::off(self, ctx),
      },
      Modules::ModCombo => match self.state {
        StateType::Tap => <Key as mod_combo::ModCombo>::tap(self, ctx),
        StateType::Hold => <Key as mod_combo::ModCombo>::hold(self, ctx),
        StateType::Idle => <Key as mod_combo::ModCombo>::idle(self, ctx),
        StateType::Off => <Key as mod_combo::ModCombo>::off(self, ctx),
      },
      Modules::TapStr => match self.state {
        StateType::Tap => <Key as mod_tapstr::TapStr>::tap(self, ctx),
        StateType::Hold => <Key as mod_tapstr::TapStr>::hold(self, ctx),
        StateType::Idle => <Key as mod_tapstr::TapStr>::idle(self, ctx),
        StateType::Off => <Key as mod_tapstr::TapStr>::off(self, ctx),
      },
      Modules::RGBKey => match self.state {
        StateType::Tap => <Key as rgb_key::RGBKey>::tap(self, ctx),
        StateType::Hold => <Key as rgb_key::RGBKey>::hold(self, ctx),
        StateType::Idle => <Key as rgb_key::RGBKey>::idle(self, ctx),
        StateType::Off => <Key as rgb_key::RGBKey>::off(self, ctx),
      },
      Modules::SendString => match self.state {
        StateType::Tap => <Key as sendstring::SendString>::tap(self, ctx),
        StateType::Hold => <Key as sendstring::SendString>::hold(self, ctx),
        StateType::Idle => <Key as sendstring::SendString>::idle(self, ctx),
        StateType::Off => <Key as sendstring::SendString>::off(self, ctx),
      },
      Modules::SendHIDRaw => match self.state {
        StateType::Tap => <Key as sendhid::SendHID>::tap(self),
        StateType::Hold => <Key as sendhid::SendHID>::hold(self),
        StateType::Idle => <Key as sendhid::SendHID>::idle(self),
        StateType::Off => <Key as sendhid::SendHID>::off(self),
      },
      Modules::LayerHold => match self.state {
        StateType::Tap => <Key as layer_hold::LayerHold>::tap(self, ctx),
        StateType::Hold => <Key as layer_hold::LayerHold>::hold(self, ctx),
        StateType::Idle => <Key as layer_hold::LayerHold>::idle(self, ctx),
        StateType::Off => <Key as layer_hold::LayerHold>::off(self, ctx),
      },
      Modules::Transparent => [None; 4],
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
  fn new(Args: Vec<&str, 4>) -> Self
  where
    Self: Sized,
    Self: Default;
  fn tap(&mut self) -> [Option<KeyCode>; 4];
  fn hold(&mut self) -> [Option<KeyCode>; 4];
  fn idle(&self) -> [Option<KeyCode>; 4];
  fn off(&mut self) -> [Option<KeyCode>; 4];
}

impl Default for Key {
  fn new(Args: Vec<&str, 4>) -> Self {
    let KC1: Option<KeyCode> = match KeyCode::try_from(Args[0]) {
      Ok(kc) => Some(kc),
      Err(_) => None,
    };
    Key {
      cycles: 0,
      raw_state: false,
      cycles_off: 0,
      state: StateType::Off,
      prevstate: StateType::Off,
      keycode: [KC1, None, None, None],
      previnfo: [false; 6],
      stor: [0; 6],
      typ: Modules::Default,
      strng: "",
    }
  }

  fn tap(&mut self) -> [Option<KeyCode>; 4] {
    if self.keycode[0].is_some() {
      let kc0 = self.keycode[0].unwrap();
      if self.prevstate == StateType::Off {
        action(CallbackActions::Press, ARGS::KS { code: kc0 });
      }
    }
    self.keycode
  }

  fn hold(&mut self) -> [Option<KeyCode>; 4] {
    if self.keycode[0].is_some() {
      let kc0 = self.keycode[0].unwrap();
      action(CallbackActions::Press, ARGS::KS { code: kc0 });
      self.keycode
    } else {
      [None; 4]
    }
  }

  fn idle(&self) -> [Option<KeyCode>; 4] { [None; 4] }

  fn off(&mut self) -> [Option<KeyCode>; 4] {
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

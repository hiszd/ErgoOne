use defmt::debug;
use heapless::Vec;

use crate::action;
use crate::actions::CallbackActions;
use crate::key::Modules;
use crate::keyscanning::StateType;
use crate::Context;
use crate::ARGS;
use crate::{key::Key, key_codes::KeyCode};

pub const MOD_STR: &str = "lyh";

pub trait LayerHold {
  fn new(s: &str) -> Self
  where
    Self: Sized,
    Self: LayerHold;
  fn tap(&mut self, ctx: Context) -> [Option<KeyCode>; 4];
  fn hold(&mut self, _ctx: Context) -> [Option<KeyCode>; 4];
  fn idle(&mut self, _ctx: Context) -> [Option<KeyCode>; 4];
  fn off(&mut self, _ctx: Context) -> [Option<KeyCode>; 4];
}

impl LayerHold for Key {
  fn new(s: &str) -> Self {
    let sr = s.split(",").map(|s| s.trim()).collect::<Vec<&str, 2>>();
    debug!("sr: {:?}", sr);
    Key {
      cycles: 0,
      raw_state: false,
      cycles_off: 0,
      state: StateType::Off,
      prevstate: StateType::Off,
      keycode: [None; 4],
      previnfo: [false; 6],
      stor: [sr[0].parse().unwrap(), sr[1].parse().unwrap(), 0, 0, 0, 0],
      typ: Modules::LayerHold,
      strng: "",
    }
  }

  fn tap(&mut self, _ctx: Context) -> [Option<KeyCode>; 4] {
    if self.prevstate != StateType::Tap {
      debug!("Tap");
      action(CallbackActions::SetLayer, ARGS::LYR { l: 1 });
      self.previnfo[0] = true;
    }
    [None; 4]
  }

  fn hold(&mut self, _ctx: Context) -> [Option<KeyCode>; 4] { [None; 4] }

  fn idle(&mut self, _ctx: Context) -> [Option<KeyCode>; 4] { [None; 4] }

  fn off(&mut self, _ctx: Context) -> [Option<KeyCode>; 4] {
    if self.previnfo[0] {
      debug!("Off: {}, {}", self.previnfo[0], self.prevstate);
      action(CallbackActions::SetLayer, ARGS::LYR { l: 0 });
      self.previnfo[0] = false;
    }
    [None; 4]
  }
}

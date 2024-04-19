use defmt::info;
use heapless::Vec;

use crate::action;
use crate::actions::CallbackActions;
use crate::key::Modules;
use crate::keyscanning::StateType;
use crate::Context;
use crate::ARGS;
use crate::{key::Key, key_codes::KeyCode};

pub trait LayerHold {
  fn new(Args: Vec<&str, 4>, layer: usize) -> Self
  where
    Self: Sized,
    Self: LayerHold;
  fn tap(&mut self, ctx: Context) -> [Option<KeyCode>; 4];
  fn hold(&mut self, _ctx: Context) -> [Option<KeyCode>; 4];
  fn idle(&mut self, _ctx: Context) -> [Option<KeyCode>; 4];
  fn off(&mut self, _ctx: Context) -> [Option<KeyCode>; 4];
  fn set_layer(&mut self, layer: usize);
}

impl LayerHold for Key {
  fn new(Args: Vec<&str, 4>, layer: usize) -> Self {
    let Stor0: u16 = match Args[0].parse() {
      Ok(v) => v,
      Err(_) => 0,
    };
    Key {
      cycles: 0,
      raw_state: false,
      cycles_off: 0,
      state: StateType::Off,
      prevstate: StateType::Off,
      keycode: [Some(KeyCode::EEEEEEEE), None, None, None],
      previnfo: [false; 6],
      stor: [Stor0, layer as u16, 0, 0, 0, 0],
      typ: Modules::LayerHold,
      strng: "",
    }
  }

  fn set_layer(&mut self, layer: usize) { self.stor[1] = layer as u16; }

  fn tap(&mut self, _ctx: Context) -> [Option<KeyCode>; 4] {
    if self.prevstate != StateType::Tap {
      info!("Tap: {}", self.stor[0]);
      action(CallbackActions::SetLayer, ARGS::LYR {
        l: self.stor[0].into(),
      });
      self.previnfo[0] = true;
    }
    [None; 4]
  }

  fn hold(&mut self, _ctx: Context) -> [Option<KeyCode>; 4] { [None; 4] }

  fn idle(&mut self, _ctx: Context) -> [Option<KeyCode>; 4] { [None; 4] }

  fn off(&mut self, _ctx: Context) -> [Option<KeyCode>; 4] {
    // TODO: use the layer info to selectively ignore off calls from layer changes(if possible)
    if self.previnfo[0] {
      info!("Off: {}, {}", self.previnfo[0], self.prevstate);
      action(CallbackActions::SetLayer, ARGS::LYR {
        l: self.stor[1].into(),
      });
      self.previnfo[0] = false;
    }
    [None; 4]
  }
}

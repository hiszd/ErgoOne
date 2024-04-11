use heapless::Vec;

use crate::action;
use crate::actions::CallbackActions;
use crate::key::Modules;
use crate::keyscanning::StateType;
use crate::Context;
use crate::ARGS;
use crate::{key::Key, key_codes::KeyCode};

pub trait RGBKey {
  fn new(Args: Vec<&str, 4>) -> Self
  where
    Self: Sized,
    Self: RGBKey;
  fn tap(&mut self, ctx: Context) -> [Option<KeyCode>; 4];
  fn hold(&mut self, _ctx: Context) -> [Option<KeyCode>; 4];
  fn idle(&mut self, _ctx: Context) -> [Option<KeyCode>; 4];
  fn off(&mut self, _ctx: Context) -> [Option<KeyCode>; 4];
}

impl RGBKey for Key {
  fn new(Args: Vec<&str, 4>) -> Self {
    let cols = Args[0].split('_').collect::<Vec<&str, 4>>();
    Key {
      cycles: 0,
      raw_state: false,
      cycles_off: 0,
      state: StateType::Off,
      prevstate: StateType::Off,
      keycode: [Some(KeyCode::EEEEEEEE), None, None, None],
      previnfo: [false; 6],
      stor: [
        cols[0].parse().unwrap(),
        cols[1].parse().unwrap(),
        cols[2].parse().unwrap(),
        0,
        0,
        0,
      ],
      typ: Modules::RGBKey,
      strng: "",
    }
  }

  fn tap(&mut self, _ctx: Context) -> [Option<KeyCode>; 4] {
    let [Some(_kc0), Some(kc1), None, None] = self.keycode else {
      return [None; 4];
    };
    action(CallbackActions::RGBSet, ARGS::RGB {
      r: self.stor[0],
      g: self.stor[1],
      b: self.stor[2],
    });
    [Some(kc1), None, None, None]
  }

  fn hold(&mut self, _ctx: Context) -> [Option<KeyCode>; 4] {
    let [Some(_kc0), Some(kc1), None, None] = self.keycode else {
      return [None; 4];
    };
    [Some(kc1), None, None, None]
  }

  fn idle(&mut self, _ctx: Context) -> [Option<KeyCode>; 4] { [None; 4] }

  fn off(&mut self, _ctx: Context) -> [Option<KeyCode>; 4] {
    let [Some(_kc0), Some(kc1), None, None] = self.keycode else {
      return [None; 4];
    };
    [Some(kc1), None, None, None]
  }
}

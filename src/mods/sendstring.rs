use heapless::Vec;

use crate::action;
use crate::actions::CallbackActions;
use crate::key::Modules;
use crate::keyscanning::StateType;
use crate::Context;
use crate::ARGS;
use crate::{key::Key, key_codes::KeyCode};

pub trait SendString {
  fn new(Args: Vec<&'static str, 4>) -> Self
  where
    Self: Sized,
    Self: SendString;
  fn tap(&mut self, ctx: Context) -> [Option<KeyCode>; 4];
  fn hold(&mut self, _ctx: Context) -> [Option<KeyCode>; 4];
  fn idle(&mut self, _ctx: Context) -> [Option<KeyCode>; 4];
  fn off(&mut self, _ctx: Context) -> [Option<KeyCode>; 4];
}

impl SendString for Key {
  fn new(Args: Vec<&'static str, 4>) -> Self {
    Key {
      cycles: 0,
      raw_state: false,
      cycles_off: 0,
      state: StateType::Off,
      prevstate: StateType::Off,
      keycode: [Some(KeyCode::EEEEEEEE), Some(KeyCode::EEEEEEEE), None, None],
      previnfo: [false; 6],
      stor: [0; 6],
      typ: Modules::SendString,
      strng: Args[0],
    }
  }

  fn tap(&mut self, _ctx: Context) -> [Option<KeyCode>; 4] {
    let [Some(_kc0), Some(kc1), None, None] = self.keycode else {
      return [None; 4];
    };
    if self.prevstate != StateType::Tap {
      action(CallbackActions::SendString, ARGS::STR {
        s: self.strng.into(),
      });
      [Some(kc1), None, None, None]
    } else {
      [None; 4]
    }
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

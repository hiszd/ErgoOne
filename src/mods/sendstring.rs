use crate::action;
use crate::actions::CallbackActions;
use crate::keyscanning::StateType;
use crate::Context;
use crate::ARGS;
use crate::{key::Key, key_codes::KeyCode};

pub const MOD_STR: &str = "sst";

pub trait SendString {
  fn new(s: &'static str) -> Self
  where
    Self: Sized,
    Self: SendString;
  fn tap(&mut self, ctx: Context) -> [Option<KeyCode>; 4];
  fn hold(&mut self, _ctx: Context) -> [Option<KeyCode>; 4];
  fn idle(&mut self, _ctx: Context) -> [Option<KeyCode>; 4];
  fn off(&mut self, _ctx: Context) -> [Option<KeyCode>; 4];
}

impl SendString for Key {
  fn new(s: &'static str) -> Self {
    Key {
      cycles: 0,
      raw_state: false,
      cycles_off: 0,
      state: StateType::Off,
      prevstate: StateType::Off,
      keycode: [Some(KeyCode::EEEEEEEE), Some(KeyCode::EEEEEEEE), None, None],
      previnfo: [false; 6],
      stor: [0; 6],
      typ: "SendString",
      strng: s,
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

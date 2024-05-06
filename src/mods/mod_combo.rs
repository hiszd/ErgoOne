use heapless::Vec;

use crate::action;
use crate::actions::CallbackActions;
use crate::key::Modules;
use crate::keyscanning::StateType;
use crate::Context;
use crate::ARGS;
use crate::{key::Key, key_codes::KeyCode};

pub trait ModCombo {
  fn new(Args: Vec<&str, 4>) -> Self
  where
    Self: Sized,
    Self: ModCombo;
  fn tap(&mut self, ctx: Context) -> [Option<KeyCode>; 4];
  fn hold(&mut self, _ctx: Context) -> [Option<KeyCode>; 4];
  fn idle(&mut self, _ctx: Context) -> [Option<KeyCode>; 4];
  fn off(&mut self, _ctx: Context) -> [Option<KeyCode>; 4];
}

impl ModCombo for Key {
  fn new(Args: Vec<&str, 4>) -> Self {
    let KC0: Option<KeyCode> = match KeyCode::try_from(Args[0]) {
      Ok(kc) => Some(kc),
      Err(_) => None,
    };
    let KC1: Option<KeyCode> = match KeyCode::try_from(Args[1]) {
      Ok(kc) => Some(kc),
      Err(_) => None,
    };
    Key {
      cycles: 0,
      raw_state: false,
      cycles_off: 0,
      state: StateType::Off,
      prevstate: StateType::Off,
      keycode: [KC0, KC1, None, None],
      previnfo: [false; 6],
      stor: [0; 6],
      typ: Modules::ModCombo,
      strng: "",
    }
  }

  fn tap(&mut self, _ctx: Context) -> [Option<KeyCode>; 4] {
    let [Some(kc0), Some(kc1), None, None] = self.keycode else {
      return [None; 4];
    };
    if kc0.is_modifier() {
      self.previnfo[1] = true;
      action(CallbackActions::Press, ARGS::KS { code: kc0 }).unwrap();
    } else {
      action(CallbackActions::Press, ARGS::KS { code: kc0 }).unwrap();
      action(CallbackActions::Press, ARGS::KS { code: kc1 }).unwrap();
    }
    if self.previnfo[1] && self.prevstate == StateType::Tap {
      action(CallbackActions::Press, ARGS::KS { code: kc1 }).unwrap();
    }

    self.stor[4] = 0;
    [Some(kc0), Some(kc1), None, None]
  }

  fn hold(&mut self, _ctx: Context) -> [Option<KeyCode>; 4] { [None; 4] }

  fn idle(&mut self, _ctx: Context) -> [Option<KeyCode>; 4] { [None; 4] }

  fn off(&mut self, _ctx: Context) -> [Option<KeyCode>; 4] {
    let [Some(kc0), Some(kc1), None, None] = self.keycode else {
      return [None; 4];
    };
    if self.previnfo[1] {
      if self.stor[4] == 1 {
        action(CallbackActions::Release, ARGS::KS { code: kc0 }).unwrap();
        self.stor[4] += 1;
      } else if self.stor[4] == 2 {
        action(CallbackActions::Release, ARGS::KS { code: kc1 }).unwrap();
        self.previnfo[1] = false;
        self.stor[4] += 1;
      } else {
        self.stor[4] += 1;
      }

      return [Some(kc0), Some(kc1), None, None];
    } else {
      if self.prevstate == StateType::Tap {
        action(CallbackActions::Release, ARGS::KS { code: kc0 }).unwrap();
        action(CallbackActions::Release, ARGS::KS { code: kc1 }).unwrap();
      }
    }
    [None; 4]
  }
}

use heapless::Vec;

use crate::action;
use crate::actions::CallbackActions;
use crate::key::Modules;
use crate::keyscanning::StateType;
use crate::Context;
use crate::ARGS;
use crate::{key::Key, key_codes::KeyCode};

pub const MOD_STR: &str = "mdc";

pub trait ModCombo {
  fn new(s: &str) -> Self
  where
    Self: Sized,
    Self: ModCombo;
  fn tap(&mut self, ctx: Context) -> [Option<KeyCode>; 4];
  fn hold(&mut self, _ctx: Context) -> [Option<KeyCode>; 4];
  fn idle(&mut self, _ctx: Context) -> [Option<KeyCode>; 4];
  fn off(&mut self, _ctx: Context) -> [Option<KeyCode>; 4];
}

impl ModCombo for Key {
  fn new(s: &str) -> Self {
    let sr = s.split(",").map(|s| s.trim()).collect::<Vec<&str, 2>>();
    Key {
      cycles: 0,
      raw_state: false,
      cycles_off: 0,
      state: StateType::Off,
      prevstate: StateType::Off,
      keycode: [Some(sr[0].into()), Some(sr[1].into()), None, None],
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
      action(CallbackActions::Press, ARGS::KS { code: kc0 });
    } else {
      action(CallbackActions::Press, ARGS::KS { code: kc0 });
      action(CallbackActions::Press, ARGS::KS { code: kc1 });
    }
    if self.previnfo[1] && self.prevstate == StateType::Tap {
      action(CallbackActions::Press, ARGS::KS { code: kc1 });
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
        action(CallbackActions::Release, ARGS::KS { code: kc0 });
        self.stor[4] += 1;
      } else if self.stor[4] == 2 {
        action(CallbackActions::Release, ARGS::KS { code: kc1 });
        self.previnfo[1] = false;
        self.stor[4] += 1;
      } else {
        self.stor[4] += 1;
      }

      return [Some(kc0), Some(kc1), None, None];
    } else {
      if self.prevstate == StateType::Tap {
        action(CallbackActions::Release, ARGS::KS { code: kc0 });
        action(CallbackActions::Release, ARGS::KS { code: kc1 });
      }
    }
    [None; 4]
  }
}

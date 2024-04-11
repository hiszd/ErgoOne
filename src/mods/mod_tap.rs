use defmt::error;
use heapless::Vec;

use crate::action;
use crate::actions::CallbackActions;
use crate::key::Modules;
use crate::keyscanning::StateType;
use crate::Context;
use crate::ARGS;
use crate::{key::Key, key_codes::KeyCode};

pub trait ModTap {
  fn new(Args: Vec<&str, 4>) -> Self
  where
    Self: Sized,
    Self: ModTap;
  fn tap(&mut self, ctx: Context) -> [Option<KeyCode>; 4];
  fn hold(&mut self, _ctx: Context) -> [Option<KeyCode>; 4];
  fn idle(&mut self, _ctx: Context) -> [Option<KeyCode>; 4];
  fn off(&mut self, _ctx: Context) -> [Option<KeyCode>; 4];
  fn exist_next(&self, ctx: Context, key: KeyCode, ignore_mods: bool) -> bool;
}

// INFO: previnfo is used for the following:
// 0: Detected that other keys were pressed while this key was pressed, or the key was held.
// 1:
// 2:
// 3:
// 4: The secondary function of this key should be, or was, activated.
// 5:
impl ModTap for Key {
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
      typ: Modules::ModTap,
      strng: "",
    }
  }

  fn tap(&mut self, ctx: Context) -> [Option<KeyCode>; 4] {
    let [Some(_kc0), Some(kc1), None, None] = self.keycode else {
      return [None; 4];
    };
    if kc1.is_modifier() {
      if self.prevstate == StateType::Off {
        self.previnfo[0] = false;
        action(CallbackActions::Press, ARGS::KS { code: kc1 });
      }
      if self.exist_next(ctx, kc1, true) {
        self.previnfo[0] = true;
      }
    } else {
      error!("{} is not a modifier", kc1);
      return [None; 4];
    }
    [Some(kc1), None, None, None]
  }

  fn hold(&mut self, _ctx: Context) -> [Option<KeyCode>; 4] {
    let [Some(_kc0), Some(kc1), None, None] = self.keycode else {
      return [None; 4];
    };
    self.previnfo[4] = false;
    self.previnfo[0] = true;
    [Some(kc1), None, None, None]
  }

  fn idle(&mut self, _ctx: Context) -> [Option<KeyCode>; 4] { [None; 4] }

  fn off(&mut self, ctx: Context) -> [Option<KeyCode>; 4] {
    let [Some(kc0), Some(kc1), None, None] = self.keycode else {
      return [None; 4];
    };
    match self.prevstate {
      StateType::Tap => {
        action(CallbackActions::Release, ARGS::KS { code: kc1 });

        if !self.exist_next(ctx, kc1, true) {
          if !self.previnfo[0] {
            self.previnfo[4] = true;
          }
        } else {
          // WARN: This might not work. Testing Required
          // NOTE: Other keys were pressed.
          self.previnfo[0] = true;
          // NOTE: Do not initiate the secondary function of this key.
          self.previnfo[4] = false;
        }

        return [Some(kc1), None, None, None];
      }
      StateType::Hold => {
        action(CallbackActions::Release, ARGS::KS { code: kc1 });
        return [Some(kc1), None, None, None];
      }
      StateType::Off => {
        if self.previnfo[4] && !self.previnfo[0] {
          match self.stor[4] {
            2 => {
              if !self.exist_next(ctx, kc0, true) {
                action(CallbackActions::Press, ARGS::KS { code: kc0 });
                self.stor[4] += 1;
              }
            }
            3 => {
              action(CallbackActions::Release, ARGS::KS { code: kc0 });
              self.stor[4] += 1;
            }
            4 => {
              self.previnfo[4] = false;
              self.stor[4] = 0;
            }
            _ => {
              self.stor[4] += 1;
            }
          }
        }
        [None; 4]
      }
      _ => {
        return [None; 4];
      }
    }
  }

  fn exist_next(&self, ctx: Context, key: KeyCode, ignore_mods: bool) -> bool {
    let mut rtrn1 = false;
    for i in 0..ctx.key_queue.len() {
      if ctx.key_queue[i].is_some() {
        if let Some(curkey) = ctx.key_queue[i] {
          if curkey != key {
            if ignore_mods {
              if !curkey.is_modifier() {
                rtrn1 = true;
                break;
              }
            } else {
              rtrn1 = true;
              break;
            }
          }
        }
      }
    }
    rtrn1
  }
}

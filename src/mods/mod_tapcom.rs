use defmt::error;
use defmt::info;
use defmt::println;
use heapless::Vec;

use crate::action;
use crate::actions::CallbackActions;
use crate::key::Modules;
use crate::keyscanning::StateType;
use crate::Context;
use crate::ARGS;
use crate::{key::Key, key_codes::KeyCode};

pub trait TapCom {
  fn new(Args: Vec<&str, 4>) -> Self
  where
    Self: Sized,
    Self: TapCom;
  fn tap(&mut self, ctx: Context) -> [Option<KeyCode>; 4];
  fn hold(&mut self, _ctx: Context) -> [Option<KeyCode>; 4];
  fn idle(&mut self, _ctx: Context) -> [Option<KeyCode>; 4];
  fn off(&mut self, _ctx: Context) -> [Option<KeyCode>; 4];
  fn exist_next(&self, ctx: Context, key: KeyCode, ignore_mods: bool) -> bool;
}

// INFO: previnfo is used for the following:
// 0: Detected that other keys were pressed, or the key was held, not tapped
// 1:
// 2:
// 3:
// 4:
// 5:
impl TapCom for Key {
  fn new(Args: Vec<&str, 4>) -> Self {
    let KC0: Option<KeyCode> = match KeyCode::try_from(Args[0]) {
      Ok(kc) => Some(kc),
      Err(_) => None,
    };
    let KC1: Option<KeyCode> = match KeyCode::try_from(Args[1]) {
      Ok(kc) => Some(kc),
      Err(_) => None,
    };
    let KC2: Option<KeyCode> = match KeyCode::try_from(Args[2]) {
      Ok(kc) => Some(kc),
      Err(_) => None,
    };
    Key {
      cycles: 0,
      raw_state: false,
      cycles_off: 0,
      state: StateType::Off,
      prevstate: StateType::Off,
      keycode: [KC0, KC1, KC2, None],
      previnfo: [false; 6],
      stor: [0; 6],
      typ: Modules::TapCom,
      strng: "",
    }
  }

  fn tap(&mut self, ctx: Context) -> [Option<KeyCode>; 4] {
    let [Some(kc0), Some(_kc1), Some(_kc2), None] = self.keycode else {
      return [None; 4];
    };
    if !self.previnfo[0] {
      if kc0.is_modifier() {
        // if there is another key pressed
        if self.exist_next(ctx, kc0, true) {
          info!("THEY DO EXIST");
          self.previnfo[0] = true;
        }
      } else {
        error!("{} is not a modifier", kc0);
        return [None; 4];
      }
    }
    if self.prevstate == StateType::Off {
      action(CallbackActions::Press, ARGS::KS { code: kc0 });
      return [Some(kc0), None, None, None];
    }
    [None; 4]
  }

  fn hold(&mut self, _ctx: Context) -> [Option<KeyCode>; 4] {
    let [Some(kc0), Some(_kc1), Some(_kc2), None] = self.keycode else {
      return [None; 4];
    };
    self.previnfo[0] = true;
    match kc0.is_modifier() {
      true => {
        action(CallbackActions::Press, ARGS::KS { code: kc0 });
      }
      false => error!("{} is not a modifier", kc0),
    }
    [Some(kc0), None, None, None]
  }

  fn idle(&mut self, _ctx: Context) -> [Option<KeyCode>; 4] { [None; 4] }

  fn off(&mut self, ctx: Context) -> [Option<KeyCode>; 4] {
    let [Some(kc0), Some(kc1), Some(kc2), None] = self.keycode else {
      return [None; 4];
    };
    match self.prevstate {
      StateType::Tap => {
        // if there was not a combination of key pressed during the tap then
        // if !self.previnfo[0] && !self.exist_next(ctx, kc0, true) {
        if !self.previnfo[0] {
          println!("no combo");
          self.previnfo[1] = true;
          self.stor[4] = 0;
          action(CallbackActions::Release, ARGS::KS { code: kc0 });
          action(CallbackActions::Press, ARGS::KS { code: kc1 });
          action(CallbackActions::Press, ARGS::KS { code: kc2 });
          return [Some(kc0), Some(kc1), Some(kc2), None];
          // if there was a combination of keys pressed then do nothing
        } else {
          println!("{}", ctx.key_queue);
          println!("combo");
          action(CallbackActions::Release, ARGS::KS { code: kc1 });
          self.previnfo[0] = false;
          return [Some(kc1), None, None, None];
        }
      }
      StateType::Hold => {
        self.previnfo[1] = false;
        action(CallbackActions::Release, ARGS::KS { code: kc0 });
        return [Some(kc0), None, None, None];
      }
      StateType::Off => {
        if self.previnfo[1] {
          if self.stor[4] == 3 {
            action(CallbackActions::Release, ARGS::KS { code: kc1 });
            action(CallbackActions::Release, ARGS::KS { code: kc2 });
            self.previnfo[1] = false;
            self.stor[4] += 1;
          } else if self.stor[4] < 5 {
            self.stor[4] += 1;
          }
        }
        return [None; 4];
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

use defmt::error;
use defmt::info;
use defmt::println;
use heapless::Vec;

use crate::action;
use crate::actions::CallbackActions;
use crate::key::DEBOUNCE_CYCLES;
use crate::key::HOLD_CYCLES;
use crate::keyscanning::StateType;
use crate::Context;
use crate::ARGS;
use crate::{key::Key, key_codes::KeyCode};

pub const MOD_STR: &str = "tpc";

pub trait TapCom {
  fn new(s: &str) -> Self
  where
    Self: Sized,
    Self: TapCom;
  fn tap(&mut self, ctx: Context) -> [Option<KeyCode>; 4];
  fn hold(&mut self, _ctx: Context) -> [Option<KeyCode>; 4];
  fn idle(&mut self, _ctx: Context) -> [Option<KeyCode>; 4];
  fn off(&mut self, _ctx: Context) -> [Option<KeyCode>; 4];
  fn get_keys(&mut self, ctx: Context) -> [Option<KeyCode>; 4];
  fn scan(&mut self, is_high: bool, ctx: Context) -> [Option<KeyCode>; 4];
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
  fn new(s: &str) -> Self {
    let sr = s.split(",").map(|s| s.trim()).collect::<Vec<&str, 3>>();
    Key {
      cycles: 0,
      raw_state: false,
      cycles_off: 0,
      state: StateType::Off,
      prevstate: StateType::Off,
      keycode: [
        Some(sr[0].into()),
        Some(sr[1].into()),
        Some(sr[2].into()),
        None,
      ],
      previnfo: [false; 6],
      stor: [0; 6],
      typ: "TapCom",
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

  fn scan(&mut self, is_high: bool, ctx: Context) -> [Option<KeyCode>; 4] {
    if self.keycode[0].is_none() && self.keycode[1].is_none() {
      return [None; 4];
    }
    if is_high {
      if self.cycles < u16::MAX {
        self.cycles += 1;
      }
      self.cycles_off = 0;
    } else {
      if self.cycles_off < u16::MAX {
        self.cycles_off += 1;
      }
      self.cycles = 0;
    }
    self.raw_state = is_high;
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

  fn get_keys(&mut self, ctx: Context) -> [Option<KeyCode>; 4] {
    match self.state {
      StateType::Tap => self.tap(ctx),
      StateType::Hold => self.hold(ctx),
      StateType::Idle => self.idle(ctx),
      StateType::Off => self.off(ctx),
    }
  }
}

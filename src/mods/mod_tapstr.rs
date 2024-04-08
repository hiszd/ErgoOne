/*
 * This module provides the following functionality:
 * When the main key is tapped and no other keys are pressed
 * the string is sent.
 * If the main key is held, or is pressed in combination with another key, it functions as normal.
 *
 * String example:
 * "tps,Mod_LSft,("
 * Requirements(each needs to be sepearated by a comma ","):
 * - "tps" the string has to start with this to identify the module.
 * - "Mod_LSFT" this can be any keycode that is a modifier.
 * - "(" this is the string that will be typed if the key is only tapped.
 */

use defmt::error;
use heapless::Vec;

use crate::action;
use crate::actions::CallbackActions;
use crate::key::DEBOUNCE_CYCLES;
use crate::key::HOLD_CYCLES;
use crate::keyscanning::StateType;
use crate::Context;
use crate::ARGS;
use crate::{key::Key, key_codes::KeyCode};

pub const MOD_STR: &str = "tps";

pub trait TapStr {
  fn new(s: &'static str) -> Self
  where
    Self: Sized,
    Self: TapStr;
  fn tap(&mut self, ctx: Context) -> [Option<KeyCode>; 4];
  fn hold(&mut self, _ctx: Context) -> [Option<KeyCode>; 4];
  fn idle(&mut self, _ctx: Context) -> [Option<KeyCode>; 4];
  fn off(&mut self, _ctx: Context) -> [Option<KeyCode>; 4];
  fn get_keys(&mut self, ctx: Context) -> [Option<KeyCode>; 4];
  fn scan(&mut self, is_high: bool, ctx: Context) -> [Option<KeyCode>; 4];
  fn exist_next(&self, ctx: Context, key: KeyCode, ignore_mods: bool) -> bool;
}

impl TapStr for Key {
  fn new(s: &'static str) -> Self {
    let sr = s.split(",").map(|s| s.trim()).collect::<Vec<&str, 2>>();
    Key {
      cycles: 0,
      raw_state: false,
      cycles_off: 0,
      state: StateType::Off,
      prevstate: StateType::Off,
      keycode: [Some(sr[0].into()), None, None, None],
      previnfo: [false; 6],
      stor: [0; 6],
      typ: "TapStr",
      strng: sr[1],
    }
  }

  fn tap(&mut self, ctx: Context) -> [Option<KeyCode>; 4] {
    let [Some(kc0), None, None, None] = self.keycode else {
      return [None; 4];
    };
    if kc0.is_modifier() {
      if self.prevstate == StateType::Off {
        self.previnfo[0] = false;
        action(CallbackActions::Press, ARGS::KS { code: kc0 });
      }
      if self.exist_next(ctx, kc0, true) {
        self.previnfo[0] = true;
      }
    } else {
      error!("{} is not a modifier", kc0);
      return [None; 4];
    }
    [Some(kc0), None, None, None]
  }

  fn hold(&mut self, _ctx: Context) -> [Option<KeyCode>; 4] {
    let [Some(kc0), None, None, None] = self.keycode else {
      return [None; 4];
    };
    self.previnfo[4] = false;
    self.previnfo[0] = true;
    [Some(kc0), None, None, None]
  }

  fn idle(&mut self, _ctx: Context) -> [Option<KeyCode>; 4] { [None; 4] }

  fn off(&mut self, ctx: Context) -> [Option<KeyCode>; 4] {
    let [Some(kc0), None, None, None] = self.keycode else {
      return [None; 4];
    };
    match self.prevstate {
      StateType::Tap => {
        action(CallbackActions::Release, ARGS::KS { code: kc0 });

        if !self.exist_next(ctx, kc0, true) {
          if !self.previnfo[0] {
            self.previnfo[4] = true;
          }
        } else {
          // NOTE: Other keys were pressed.
          self.previnfo[0] = true;
          // NOTE: Do not initiate the secondary function of this key.
          self.previnfo[4] = false;
        }

        return [Some(kc0), None, None, None];
      }
      StateType::Hold => {
        action(CallbackActions::Release, ARGS::KS { code: kc0 });
        return [Some(kc0), None, None, None];
      }
      StateType::Off => {
        if self.previnfo[4] && !self.previnfo[0] {
          match self.stor[4] {
            2 => {
              if !self.exist_next(ctx, kc0, true) {
                action(CallbackActions::SendString, ARGS::STR {
                  s: self.strng.into(),
                });
                self.stor[4] += 1;
              }
            }
            3 => {
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

  #[doc = " Perform state change as a result of the scan"]
  fn scan(&mut self, is_high: bool, ctx: Context) -> [Option<KeyCode>; 4] {
  }
}

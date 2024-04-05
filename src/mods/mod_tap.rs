use defmt::error;
use defmt::println;
use defmt::warn;
use heapless::Vec;

use crate::action;
use crate::actions::CallbackActions;
use crate::key::DEBOUNCE_CYCLES;
use crate::key::HOLD_CYCLES;
use crate::keyscanning::StateType;
use crate::Context;
use crate::ARGS;
use crate::{key::Key, key_codes::KeyCode};

pub trait ModTap {
  fn mdtnew(s: &str) -> Self
  where
    Self: Sized,
    Self: ModTap;
  fn mdttap(&mut self, ctx: Context) -> [Option<KeyCode>; 4];
  fn mdthold(&mut self, _ctx: Context) -> [Option<KeyCode>; 4];
  fn mdtidle(&mut self, _ctx: Context) -> [Option<KeyCode>; 4];
  fn mdtoff(&mut self, _ctx: Context) -> [Option<KeyCode>; 4];
  fn get_keys(&mut self, ctx: Context) -> [Option<KeyCode>; 4];
  fn mdtscan(&mut self, is_high: bool, ctx: Context) -> [Option<KeyCode>; 4];
  fn exist_next(&self, ctx: Context, key: KeyCode, ignore_mods: bool) -> bool;
}

// INFO: previnfo is used for the following:
// 0: Detected that other keys were pressed, or the key was held, not tapped
// 1:
// 2:
// 3:
// 4:
// 5:
impl ModTap for Key {
  fn mdtnew(s: &str) -> Self {
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
      typ: "ModTap",
      strng: "",
    }
  }

  fn mdttap(&mut self, ctx: Context) -> [Option<KeyCode>; 4] {
    let [Some(_kc0), Some(kc1), None, None] = self.keycode else {
      return [None; 4];
    };
    if kc1.is_modifier() {
      if self.prevstate == StateType::Off {
        self.previnfo[0] = false;
        action(CallbackActions::Press, ARGS::KS { code: kc1 });
      }
      if self.exist_next(ctx, kc1, true) {
        // NOTE: flag that other keys were pressed while this one was pressed
        self.previnfo[0] = true;
      }
    } else {
      error!("{} is not a modifier", kc1);
      return [None; 4];
    }
    [Some(kc1), None, None, None]
  }

  fn mdthold(&mut self, _ctx: Context) -> [Option<KeyCode>; 4] {
    let [Some(_kc0), Some(kc1), None, None] = self.keycode else {
      return [None; 4];
    };
    self.previnfo[0] = true;
    [Some(kc1), None, None, None]
  }

  fn mdtidle(&mut self, _ctx: Context) -> [Option<KeyCode>; 4] { [None; 4] }

  fn mdtoff(&mut self, ctx: Context) -> [Option<KeyCode>; 4] {
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
          // This might not work. Testing Required
          self.previnfo[0] = true;
        }

        return [Some(kc1), None, None, None];
      }
      StateType::Hold => {
        action(CallbackActions::Release, ARGS::KS { code: kc1 });
        self.previnfo[4] = false;
        return [Some(kc1), None, None, None];
      }
      StateType::Off => {
        if self.previnfo[4] {
          match self.stor[4] {
            2 => {
              if !self.previnfo[0] && !self.exist_next(ctx, kc0, true) {
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
    // TODO: check if key is the comparable opposite of the one pressed(lshift to rshift, etc...)
    let mut rtrn1 = false;
    // locate key in array
    let ind1: Option<usize> = ctx
      .key_queue
      .iter()
      .position(|k| k.is_some() && k.unwrap() == key);
    let mut srt: usize = 0;
    if ind1.is_some() {
      srt = ind1.unwrap();
    }
    for i in srt..ctx.key_queue.len() {
      if ctx.key_queue[i].is_some() {
        if let Some(curkey) = ctx.key_queue[i] {
          if curkey != key {
            if ignore_mods {
              if curkey.is_modifier() {
                warn!("rtrn1 = {}, key = {}", rtrn1, ctx.key_queue[i].unwrap());
                break;
              } else {
                rtrn1 = true;
                warn!("rtrn1 = {}, key = {}", rtrn1, ctx.key_queue[i].unwrap());
                break;
              }
            } else {
              rtrn1 = true;
              warn!("rtrn1 = {}, key = {}", rtrn1, ctx.key_queue[i].unwrap());
              break;
            }
          }
        }
      }
    }
    if !rtrn1 {
      warn!("rtrn1 = false, key = ''");
    }
    rtrn1
  }

  #[doc = " Perform state change as a result of the scan"]
  fn mdtscan(&mut self, is_high: bool, ctx: Context) -> [Option<KeyCode>; 4] {
    let [Some(kc0), Some(kc1), None, None] = self.keycode else {
      return [None; 4];
    };
    if kc0 == KeyCode::________ && kc1 == KeyCode::________ {
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
      StateType::Tap => self.mdttap(ctx),
      StateType::Hold => self.mdthold(ctx),
      StateType::Idle => self.mdtidle(ctx),
      StateType::Off => self.mdtoff(ctx),
    }
  }
}

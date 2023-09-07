use defmt::debug;
use heapless::Vec;

use crate::action;
use crate::actions::CallbackActions;
use crate::key::DEBOUNCE_CYCLES;
use crate::key::HOLD_CYCLES;
use crate::keyscanning::StateType;
use crate::Context;
use crate::ARGS;
use crate::{key::Key, key_codes::KeyCode};

pub trait ModCombo {
  fn mdcnew(s: &str) -> Self
  where
    Self: Sized,
    Self: ModCombo;
  fn mdctap(&mut self, ctx: Context) -> [Option<KeyCode>; 4];
  fn mdchold(&mut self, _ctx: Context) -> [Option<KeyCode>; 4];
  fn mdcidle(&mut self, _ctx: Context) -> [Option<KeyCode>; 4];
  fn mdcoff(&mut self, _ctx: Context) -> [Option<KeyCode>; 4];
  fn get_keys(&mut self, ctx: Context) -> [Option<KeyCode>; 4];
  fn mdcscan(&mut self, is_high: bool, ctx: Context) -> [Option<KeyCode>; 4];
}

impl ModCombo for Key {
  fn mdcnew(s: &str) -> Self {
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
      typ: "ModCombo",
      strng: "",
    }
  }

  fn mdctap(&mut self, _ctx: Context) -> [Option<KeyCode>; 4] {
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

  fn mdchold(&mut self, _ctx: Context) -> [Option<KeyCode>; 4] { [None; 4] }

  fn mdcidle(&mut self, _ctx: Context) -> [Option<KeyCode>; 4] { [None; 4] }

  fn mdcoff(&mut self, _ctx: Context) -> [Option<KeyCode>; 4] {
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

  fn mdcscan(&mut self, is_high: bool, ctx: Context) -> [Option<KeyCode>; 4] {
    let [Some(kc0), Some(kc1), None, None] = self.keycode else {
            return [None; 4];
        };
    // println!("{}", is_high);
    // if they KeyCode is empty then don't bother processing
    if kc0 == KeyCode::________ && kc1 == KeyCode::________ {
      return [None; 4];
    }
    //     ____________________________
    //    |                            |
    //    |       Cycle Counters       |
    //    |                            |
    //    |____________________________|
    // set the raw state to the state of the pin
    if is_high {
      // increment cycles while pin is high
      if self.cycles < u16::MAX {
        self.cycles += 1;
      }
      self.cycles_off = 0;
    } else {
      // increment cycles_off while pin is low
      if self.cycles_off < u16::MAX {
        self.cycles_off += 1;
      }
      // reset cycles since pin is low
      self.cycles = 0;
    }
    self.raw_state = is_high;
    //     ____________________________
    //    |                            |
    //    |        State Change        |
    //    |                            |
    //    |____________________________|
    // if we have gotten more cycles in than the debounce_cycles
    if self.cycles >= DEBOUNCE_CYCLES {
      // if the current state is Tap  and we have more cycles than hold_cycles
      if self.state == StateType::Tap && self.cycles >= HOLD_CYCLES {
        self.prevstate = self.state;
        self.state = StateType::Hold;
      } else if self.state == StateType::Off || self.state == StateType::Tap {
        // if the current state is Off
        self.prevstate = self.state;
        self.state = StateType::Tap;
      } else if self.state == StateType::Hold {
        self.prevstate = self.state;
        self.state = StateType::Hold;
      }
      return self.get_keys(ctx);
    // } else if self.cycles_off >= DEBOUNCE_CYCLES.into() {
    } else if self.cycles_off >= 1 {
      self.prevstate = self.state;
      self.state = StateType::Off;
    }
    self.get_keys(ctx)
  }

  fn get_keys(&mut self, ctx: Context) -> [Option<KeyCode>; 4] {
    match self.state {
      StateType::Tap => self.mdctap(ctx),
      StateType::Hold => self.mdchold(ctx),
      StateType::Idle => self.mdcidle(ctx),
      StateType::Off => self.mdcoff(ctx),
    }
  }
}

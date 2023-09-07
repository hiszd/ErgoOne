use crate::action;
use crate::actions::CallbackActions;
use crate::key::DEBOUNCE_CYCLES;
use crate::key::HOLD_CYCLES;
use crate::keyscanning::StateType;
use crate::Context;
use crate::ARGS;
use crate::{key::Key, key_codes::KeyCode};

pub trait SendString {
  fn sstnew(s: &'static str) -> Self
  where
    Self: Sized,
    Self: SendString;
  fn ssttap(&mut self, ctx: Context) -> [Option<KeyCode>; 4];
  fn ssthold(&mut self, _ctx: Context) -> [Option<KeyCode>; 4];
  fn sstidle(&mut self, _ctx: Context) -> [Option<KeyCode>; 4];
  fn sstoff(&mut self, _ctx: Context) -> [Option<KeyCode>; 4];
  fn get_keys(&mut self, ctx: Context) -> [Option<KeyCode>; 4];
  fn sstscan(&mut self, is_high: bool, ctx: Context) -> [Option<KeyCode>; 4];
}

impl SendString for Key {
  fn sstnew(s: &'static str) -> Self {
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

  fn ssttap(&mut self, _ctx: Context) -> [Option<KeyCode>; 4] {
    let [Some(_kc0), Some(kc1), None, None] = self.keycode else {
      return [None; 4];
    };
    action(CallbackActions::SendString, ARGS::STR {
        s: self.strng.into(),
    });
    [Some(kc1), None, None, None]
  }

  fn ssthold(&mut self, _ctx: Context) -> [Option<KeyCode>; 4] {
    let [Some(_kc0), Some(kc1), None, None] = self.keycode else {
      return [None; 4];
    };
    [Some(kc1), None, None, None]
  }

  fn sstidle(&mut self, _ctx: Context) -> [Option<KeyCode>; 4] { [None; 4] }

  fn sstoff(&mut self, _ctx: Context) -> [Option<KeyCode>; 4] {
    let [Some(_kc0), Some(kc1), None, None] = self.keycode else {
      return [None; 4];
    };
    [Some(kc1), None, None, None]
  }

  fn sstscan(&mut self, is_high: bool, ctx: Context) -> [Option<KeyCode>; 4] {
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
      StateType::Tap => self.ssttap(ctx),
      StateType::Hold => self.ssthold(ctx),
      StateType::Idle => self.sstidle(ctx),
      StateType::Off => self.sstoff(ctx),
    }
  }
}

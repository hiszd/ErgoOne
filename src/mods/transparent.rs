use crate::key::{Key, Modules};
use crate::keyscanning::StateType;

pub trait Transparent {
  fn new() -> Self
  where
    Self: Sized,
    Self: Transparent;
}

impl Transparent for Key {
  fn new() -> Self {
    Key {
      cycles: 0,
      raw_state: false,
      cycles_off: 0,
      state: StateType::Off,
      prevstate: StateType::Off,
      keycode: [None; 4],
      previnfo: [false; 6],
      stor: [0; 6],
      typ: Modules::Transparent,
      strng: "",
    }
  }
}

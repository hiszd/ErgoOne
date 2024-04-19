use defmt::error;
use defmt::info;
use heapless::Vec;
use kiibohd_hid_io::h0060;

use crate::action;
use crate::actions::CallbackActions;
use crate::key::Modules;
use crate::keyscanning::StateType;
use crate::Context;
use crate::ARGS;
use crate::{key::Key, key_codes::KeyCode};

pub trait HIDVol {
  fn new(Args: Vec<&'static str, 4>) -> Self
  where
    Self: Sized,
    Self: HIDVol;
  fn tap(&mut self) -> [Option<KeyCode>; 4];
  fn hold(&mut self) -> [Option<KeyCode>; 4];
  fn idle(&mut self) -> [Option<KeyCode>; 4];
  fn off(&mut self) -> [Option<KeyCode>; 4];
}

impl HIDVol for Key {
  fn new(Args: Vec<&'static str, 4>) -> Self {
    if Args.len() != 1 {
      panic!("SendHID requires 1 argument");
    }
    let value = Args[0].split(':').collect::<Vec<&str, 2>>();
    let command = h0060::Command::try_from(value[0]).unwrap();
    Key {
      cycles: 0,
      raw_state: false,
      cycles_off: 0,
      state: StateType::Off,
      prevstate: StateType::Off,
      keycode: [Some(KeyCode::EEEEEEEE), None, None, None],
      previnfo: [false; 6],
      stor: [u16::try_from(command).unwrap(),value[1].parse::<u16>().unwrap(),0,0,0,0],
      typ: Modules::HIDVol,
      strng: "",
    }
  }

  fn tap(&mut self) -> [Option<KeyCode>; 4] {
    if self.prevstate != StateType::Tap {
      info!("HIDVol: {}", self.strng);
      action(CallbackActions::HIDVol, ARGS::VOL { command: h0060::Command::try_from(self.stor[0]).unwrap(), vol: self.stor[1].try_into().unwrap() });
    }
    [None; 4]
  }

  fn hold(&mut self) -> [Option<KeyCode>; 4] { [None; 4] }

  fn idle(&mut self) -> [Option<KeyCode>; 4] { [None; 4] }

  fn off(&mut self) -> [Option<KeyCode>; 4] { [None; 4] }
}

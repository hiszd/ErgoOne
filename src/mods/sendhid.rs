use defmt::error;
use defmt::info;
use heapless::Vec;

use crate::action;
use crate::actions::CallbackActions;
use crate::key::Modules;
use crate::keyscanning::StateType;
use crate::Context;
use crate::ARGS;
use crate::{key::Key, key_codes::KeyCode};

pub trait SendHID {
  fn new(Args: Vec<&'static str, 4>) -> Self
  where
    Self: Sized,
    Self: SendHID;
  fn tap(&mut self) -> [Option<KeyCode>; 4];
  fn hold(&mut self) -> [Option<KeyCode>; 4];
  fn idle(&mut self) -> [Option<KeyCode>; 4];
  fn off(&mut self) -> [Option<KeyCode>; 4];
}

impl SendHID for Key {
  fn new(Args: Vec<&'static str, 4>) -> Self {
    if Args.len() != 1 {
      panic!("SendHID requires 1 argument");
    }
    Key {
      cycles: 0,
      raw_state: false,
      cycles_off: 0,
      state: StateType::Off,
      prevstate: StateType::Off,
      keycode: [Some(KeyCode::EEEEEEEE), None, None, None],
      previnfo: [false; 6],
      stor: [0; 6],
      typ: Modules::SendHIDRaw,
      strng: Args[0],
    }
  }

  fn tap(&mut self) -> [Option<KeyCode>; 4] {
    if self.prevstate != StateType::Tap {
      info!("SendHIDRaw: {}", self.strng);
      action(CallbackActions::Press, ARGS::KS {
        code: KeyCode::Fun_Escz,
      });
      action(CallbackActions::Release, ARGS::KS {
        code: KeyCode::Fun_Escz,
      });
      action(CallbackActions::SendHIDRaw, ARGS::HID { data: self.strng });
    }
    [None; 4]
  }

  fn hold(&mut self) -> [Option<KeyCode>; 4] { [None; 4] }

  fn idle(&mut self) -> [Option<KeyCode>; 4] { [None; 4] }

  fn off(&mut self) -> [Option<KeyCode>; 4] { [None; 4] }
}

// #[allow(dead_code)]
// struct HIDQUEUE {
//   queue: [&'static strl 5],
// }

// #[allow(dead_code)]
// impl HIDQUEUE {
//   fn new() -> Self { HIDQUEUE { queue: Vec::new() } }
//   pub fn push(&mut self, data: &'static str) {
//     match self.queue.push(data) {
//       Ok(_) => {}
//       Err(_) => error!("HIDQUEUE is full"),
//     };
//   }
//   pub fn take(&mut self) -> Option<&'static str> {
//     if self.queue.len() > 0 {
//       let thng = Some(self.queue[0]);
//       if self.queue.len() > 1 {
//         for i in 0..self.queue.len() - 1 {
//           if i == self.queue.len() - 1 {
//           } else {
//             self.queue[i] = self.queue[i + 1];
//           }
//         }
//       }
//       return thng;
//     }
//     None
//   }
// }

use defmt::info;
use heapless::{String, Vec};

use crate::actions::CallbackActions;
use crate::key_codes::KeyCode;
use crate::{action, ARGS};

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct Event {
  pub typ: EventType,
  pub args: ARGS,
}

impl defmt::Format for Event {
  fn format(&self, f: defmt::Formatter) {
    defmt::write!(f, "Event: {{ typ: {:?}, args: {:?} }}", self.typ, self.args);
  }
}

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub enum EventType {
  LayerSet,
  KeyDown,
  KeyUp,
  StringSent,
  RgbSet,
  HidRawSent,
}

impl defmt::Format for EventType {
  fn format(&self, f: defmt::Formatter) {
    match *self {
      EventType::LayerSet => defmt::write!(f, "EventType::LayerSet"),
      EventType::KeyDown => defmt::write!(f, "EventType::KeyDown"),
      EventType::KeyUp => defmt::write!(f, "EventType::KeyUp"),
      EventType::StringSent => defmt::write!(f, "EventType::StringSent"),
      EventType::RgbSet => defmt::write!(f, "EventType::RgbSet"),
      EventType::HidRawSent => defmt::write!(f, "EventType::HidRawSent"),
    }
  }
}

pub struct Emitter {
  prev_events: Vec<Option<Event>, 4>,
}

/// Emit events
/// Put things in these functions that you want to have happen on certain events
impl Emitter {
  pub const fn new() -> Self {
    Self {
      prev_events: Vec::new(),
    }
  }
  pub fn call(&mut self, event: EventType, args: ARGS) {
    match event {
      EventType::LayerSet => match args {
        ARGS::LYR { l: _ } => self.layer_set(args),
        _ => info!("Expected ARGS::LYR but got something else"),
      },
      EventType::KeyDown => match args {
        ARGS::KS { code: _ } => self.key_down(args),
        _ => info!("Expected ARGS::KS but got something else"),
      },
      EventType::KeyUp => match args {
        ARGS::KS { code: _ } => self.key_up(args),
        _ => info!("Expected ARGS::KS but got something else"),
      },
      EventType::StringSent => match args {
        ARGS::STR { s: _ } => self.string_sent(args),
        _ => info!("Expected ARGS::KS but got something else"),
      },
      EventType::RgbSet => match args {
        ARGS::RGB { r: _, g: _, b: _ } => self.rgb_set(args),
        _ => info!("Expected ARGS::RGB but got something else"),
      },
      EventType::HidRawSent => match args {
        ARGS::HID { data: _ } => self.hid_raw_sent(args.clone()),
        _ => info!("Expected ARGS::HID but got something else"),
      },
    }
  }
  pub fn push(&mut self, event: Event) {
    let mut prev_events = self.prev_events.clone();
    if prev_events.len() == 0 {
      let _ = prev_events.push(Some(event.clone()));
    } else {
      if prev_events.len() != prev_events.capacity() {
        let _ = prev_events.push(None);
      }
      let len = prev_events.len();
      for i in 0..len {
        let i = len - i - 1;
        if i == 0 {
          prev_events[i] = Some(event.clone());
        } else {
          if prev_events[i - 1].is_some() {
            prev_events[i] = prev_events[i - 1].clone();
          }
        }
      }
    }
    self.prev_events = prev_events;
  }

  fn layer_set(&mut self, args: ARGS) {
    self.push(Event {
      typ: EventType::LayerSet,
      args: args.clone(),
    });
  }
  fn key_down(&mut self, args: ARGS) {
    self.push(Event {
      typ: EventType::KeyDown,
      args: args.clone(),
    });
  }
  fn key_up(&mut self, args: ARGS) {
    match args {
      ARGS::KS { code } => {
          match code {
              KeyCode::Num_1zzz => {
                  action(CallbackActions::SendHIDRaw, ARGS::HID {
                      data: String::from("volume:100"),
                  });
              }
              KeyCode::Num_2zzz => {
                  action(CallbackActions::SendHIDRaw, ARGS::HID {
                      data: String::from("volume:0"),
                  });
              }
              _ => {}
          }
      }
      _ => info!("Expected ARGS::KS but got something else"),
    }
    self.push(Event {
      typ: EventType::KeyUp,
      args: args.clone(),
    });
  }
  fn string_sent(&mut self, args: ARGS) {
    self.push(Event {
      typ: EventType::StringSent,
      args: args.clone(),
    });
  }
  fn rgb_set(&mut self, args: ARGS) {
    self.push(Event {
      typ: EventType::RgbSet,
      args: args.clone(),
    });
  }
  fn hid_raw_sent(&mut self, args: ARGS) {
    self.push(Event {
      typ: EventType::HidRawSent,
      args: args.clone(),
    })
  }
}

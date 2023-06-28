#![allow(unused_imports)]
use defmt::export::debug;
use defmt::{info, println};

use crate::{key_codes::KeyCode, keyscanning::StateType};

const DEBOUNCE_CYCLES: u16 = 3;
const HOLD_CYCLES: u16 = 30;
// TODO impl idle tracking
// const IDLE_CYCLES: u8 = 100;

// #[derive(Copy, Clone, PartialEq, PartialOrd)]
#[derive(Copy, Clone)]
pub struct Key {
    pub cycles: u16,
    pub raw_state: bool,
    pub cycles_off: u16,
    pub state: StateType,
    pub prevstate: StateType,
    pub keycode: [KeyCode; 2],
    pub tap: fn([KeyCode; 2]) -> [KeyCode; 2],
    pub hold: fn([KeyCode; 2]) -> [KeyCode; 2],
    pub idle: fn([KeyCode; 2]) -> [KeyCode; 2],
    pub off: fn([KeyCode; 2]) -> [KeyCode; 2],
}

pub trait Default {
    fn new(KC1: KeyCode, KC2: Option<KeyCode>) -> Self
    where
        Self: Sized;
}

impl Default for Key {
    fn new(KC1: KeyCode, KC2: Option<KeyCode>) -> Self {
        Key {
            cycles: 0,
            raw_state: false,
            cycles_off: 0,
            state: StateType::Off,
            prevstate: StateType::Off,
            keycode: [KC1, KC2.unwrap_or(KeyCode::________)],
            tap: |keycodes: [KeyCode; 2]| keycodes,
            hold: |keycodes: [KeyCode; 2]| keycodes,
            idle: |_keycodes: [KeyCode; 2]| [KeyCode::________, KeyCode::________],
            off: |_keycodes: [KeyCode; 2]| [KeyCode::________, KeyCode::________],
        }
    }
}

#[allow(dead_code)]
impl Key {
    /// Perform state change as a result of the scan
    pub fn scan(&mut self, is_high: bool) -> [KeyCode; 2] {
        // println!("{}", is_high);
        const DEFCODE: [KeyCode; 2] = [KeyCode::________, KeyCode::________];
        // if they KeyCode is empty then don't bother processing
        if self.keycode == [KeyCode::________, KeyCode::________] {
            return DEFCODE;
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
            }
            return self.get_keys();
        // } else if self.cycles_off >= DEBOUNCE_CYCLES.into() {
        } else if self.cycles_off >= 1 {
            self.prevstate = self.state;
            self.state = StateType::Off;
        }
        self.get_keys()
    }
    pub fn get_keys(&self) -> [KeyCode; 2] {
        // info!("{:?}", self.state);
        // Match all types of self.state
        match self.state {
            StateType::Tap => (self.tap)(self.keycode),
            StateType::Hold => (self.hold)(self.keycode),
            StateType::Idle => (self.idle)(self.keycode),
            StateType::Off => (self.off)(self.keycode),
        }
    }
}

#![allow(dead_code)]
#![allow(unused_imports)]

use crate::actions::CallbackActions;
use crate::mods::mod_combo::ModCombo;
use crate::mods::mod_tap::ModTap;
use crate::mods::mod_tapcom::TapCom;
use crate::mods::rgb_key::RGBKey;
use defmt::{debug, error, info, println, warn, Format};
use embedded_hal::digital::v2::{InputPin, OutputPin};
use rp2040_hal::gpio::DynPin;
use usbd_hid::descriptor::KeyboardReport;

use crate::key::Default;
use crate::{key::Key, key_codes::KeyCode};
use crate::{Context, ARGS};

#[derive(Copy, Clone, PartialEq, PartialOrd, Debug, Format)]
pub enum StateType {
    Tap = 0,
    Hold = 1,
    Idle = 2,
    Off = 3,
}

pub struct Col {
    output: DynPin,
}

impl Col {
    pub fn new(output: DynPin) -> Self {
        let mut r: Col = Col { output };
        r.output.into_push_pull_output();
        r
    }
    pub fn set_high(&mut self) {
        self.output.set_high().unwrap()
    }
    pub fn set_low(&mut self) {
        self.output.set_low().unwrap()
    }
}

pub struct Row {
    input: DynPin,
}

impl Row {
    pub fn new(input: DynPin) -> Self {
        let mut r: Row = Row { input };
        r.input.into_floating_input();
        r
    }
    pub fn is_high(&mut self) -> bool {
        self.input.is_high().unwrap()
    }
    pub fn is_low(&mut self) -> bool {
        self.input.is_low().unwrap()
    }
    pub fn drain(&mut self) {
        self.input.into_push_pull_output();
        self.input.set_low().unwrap();
        self.input.into_floating_input();
    }
}

#[derive(Copy, Clone)]
pub struct KeyMatrix<const RSIZE: usize, const CSIZE: usize> {
    matrix: [[Key; CSIZE]; RSIZE],
}

impl<const RSIZE: usize, const CSIZE: usize> KeyMatrix<RSIZE, CSIZE> {
    pub fn new(keymap: [[Key; CSIZE]; RSIZE]) -> Self {
        KeyMatrix { matrix: keymap }
    }
}

pub struct Matrix<const RSIZE: usize, const CSIZE: usize> {
    rows: [Row; RSIZE],
    cols: [Col; CSIZE],
    layer_active: usize,
    state: [KeyMatrix<RSIZE, CSIZE>; 2],
    callback:
        fn(row: usize, col: usize, state: StateType, prevstate: StateType, keycodes: [KeyCode; 2]),
    wait_cycles: u16,
    cycles: u16,
    cur_strobe: usize,
}

impl<const RSIZE: usize, const CSIZE: usize> Matrix<RSIZE, CSIZE> {
    pub fn new(
        rows: [Row; RSIZE],
        cols: [Col; CSIZE],
        callback: fn(
            row: usize,
            col: usize,
            state: StateType,
            prevstate: StateType,
            keycodes: [KeyCode; 2],
        ),
        keymap: [KeyMatrix<RSIZE, CSIZE>; 2],
    ) -> Self {
        let mut new = Matrix {
            rows,
            cols,
            state: keymap,
            callback,
            wait_cycles: 2,
            cycles: 0,
            layer_active: 0,
            cur_strobe: (CSIZE - 1),
        };
        new.cols[new.cur_strobe].set_high();
        new.clear();
        new
    }

    fn execute_callback(
        &self,
        row: usize,
        col: usize,
        state: StateType,
        prevstate: StateType,
        keycodes: [KeyCode; 2],
    ) {
        (self.callback)(row, col, state, prevstate, keycodes);
    }

    fn clear(&mut self) {
        for r in self.cols.iter_mut() {
            r.set_low();
        }
    }

    fn next_strobe(&mut self) {
        // Unset current strobe
        self.cols[self.cur_strobe].set_low();

        // Drain stray potential from sense lines
        for c in self.rows.iter_mut() {
            c.drain();
        }

        // Check overflow condition
        if self.cur_strobe >= CSIZE - 1 {
            self.cur_strobe = 0;
        } else {
            // Increment current strobe
            self.cur_strobe += 1;
        }

        // Set new strobe as high
        self.cols[self.cur_strobe].set_high();
    }

    pub fn poll(&mut self, ctx: Context) -> bool {
        self.next_strobe();
        let c = self.cur_strobe;

        for r in 0..RSIZE {
            let codes: [Option<KeyCode>; 4];
            let _typ: &str;
            if self.state[self.layer_active].matrix[r][c].keycode[0] != Some(KeyCode::________)
                || self.state[0].matrix[r][c].keycode[0] != None
            {
                match self.state[self.layer_active].matrix[r][c].typ {
                    "Default" => {
                        codes = self.state[self.layer_active].matrix[r][c]
                            .scan(self.rows[r].is_high(), ctx);
                    }
                    "ModTap" => {
                        codes = self.state[self.layer_active].matrix[r][c]
                            .mtscan(self.rows[r].is_high(), ctx);
                    }
                    "TapCom" => {
                        codes = self.state[self.layer_active].matrix[r][c]
                            .tcscan(self.rows[r].is_high(), ctx);
                    }
                    "ModCombo" => {
                        codes = self.state[self.layer_active].matrix[r][c]
                            .mcscan(self.rows[r].is_high(), ctx);
                    }
                    "RGBKey" => {
                        codes = self.state[self.layer_active].matrix[r][c]
                            .rkscan(self.rows[r].is_high(), ctx);
                    }
                    _ => {
                        codes = [None; 4];
                        error!(
                            "Unknown key type {}",
                            self.state[self.layer_active].matrix[r][c].typ
                        );
                    }
                }
                if self.state[self.layer_active].matrix[r][c].state
                    != self.state[self.layer_active].matrix[r][c].prevstate
                {
                    self.execute_callback(
                        r + 1,
                        c + 1,
                        self.state[self.layer_active].matrix[r][c].state,
                        self.state[self.layer_active].matrix[r][c].prevstate,
                        // [KeyCode::________, KeyCode::________],
                        [
                            codes[0].unwrap_or(KeyCode::________),
                            codes[1].unwrap_or(KeyCode::________),
                        ],
                    );
                }
            }
        }
        if self.state[self.layer_active].matrix[0][0].raw_state {
            return true;
        }
        false
    }

    pub fn set_layer(&mut self, layer: usize) {
        self.layer_active = layer;
    }

    pub fn get_layer(&self) -> usize {
        self.layer_active
    }
}

#[derive(Copy, Clone)]
pub struct KeyQueue<const QSIZE: usize> {
    pub keys: [Option<KeyCode>; QSIZE],
}

impl<const QSIZE: usize> KeyQueue<QSIZE> {
    pub const fn new() -> Self {
        KeyQueue {
            keys: [None; QSIZE],
        }
    }

    pub fn len(&self) -> usize {
        self.keys.iter().filter(|k| k.is_some()).count()
    }

    pub fn is_empty(&self) -> bool {
        self.keys.iter().all(|k| k.is_none())
    }

    pub fn clear(&mut self) {
        self.keys.iter_mut().for_each(|k| {
            *k = None;
        })
    }

    /// remove all instances of a specific KeyCode
    pub fn dequeue(&mut self, key: KeyCode) -> bool {
        let mut rtrn: bool = false;
        self.keys.iter_mut().for_each(|k| {
            if k.is_some() && k.unwrap() == key {
                *k = None;
                rtrn = true;
            }
        });
        rtrn
    }

    /// push a key into the queue
    /// returns false if the queue is full
    /// returns false if the key is already in the queue
    /// returns true if the key is not in the queue
    pub fn enqueue(&mut self, key: KeyCode) -> bool {
        if self.len() >= QSIZE {
            return false;
        }
        if self.keys.iter().any(|k| k.is_some() && k.unwrap() == key) {
            false
        } else {
            for i in 0..QSIZE {
                if self.keys[i].is_none() {
                    self.keys[i] = Some(key);
                    break;
                }
            }
            true
        }
    }

    // return an array of the keys in the queue as u8s
    // returns None if the queue is empty
    pub fn get_hidcodes(&self) -> [u8; QSIZE] {
        let mut keys: [u8; QSIZE] = [0x00; QSIZE];
        self.keys.iter().enumerate().for_each(|(i, k)| {
            if k.is_some() {
                keys[i] = k.unwrap().into();
            } else {
                keys[i] = 0x00;
            }
        });
        keys
    }

    // return an array of the keys in the queue
    // returns None if the queue is empty
    pub fn get_keys(&self) -> [Option<KeyCode>; QSIZE] {
        if self.len() == 0 {
            return [None; QSIZE];
        }
        let mut keys: [Option<KeyCode>; QSIZE] = [None; QSIZE];
        self.keys.iter().enumerate().for_each(|(i, k)| {
            if let Some(k) = k {
                keys[i] = Some(*k);
            }
        });
        keys
    }
}

#![allow(dead_code)]
#![allow(unused_imports)]

// use rp2040_hal::gpio::PinMode::{Input, Output}

use defmt::{debug, info, println, Format};
use embedded_hal::digital::v2::{InputPin, OutputPin};
use rp2040_hal::gpio::DynPin;
use usbd_hid::descriptor::KeyboardReport;

use crate::key::Default;
use crate::{
    key::Key,
    key_codes::KeyCode,
    // mods::mod_tap::ModTap,
};

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
        Row { input }
    }
    pub fn is_high(&mut self) -> bool {
        self.input.into_floating_input();
        self.input.is_high().unwrap()
    }
    pub fn is_low(&mut self) -> bool {
        self.input.into_floating_input();
        self.input.is_low().unwrap()
    }
    pub fn drain(&mut self) {
        self.input.into_push_pull_output();
        self.input.set_low().unwrap()
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
    state: KeyMatrix<RSIZE, CSIZE>,
    callback:
        fn(row: usize, col: usize, state: StateType, prevstate: StateType, keycodes: [KeyCode; 2]),
    push_input: fn(c: (KeyCode, StateType)),
    mod_update: fn(c: (KeyCode, StateType)),
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
        push_input: fn(c: (KeyCode, StateType)),
        mod_update: fn(c: (KeyCode, StateType)),
        keymap: KeyMatrix<RSIZE, CSIZE>,
    ) -> Self {
        let mut new = Matrix {
            rows,
            cols,
            // state: KeyMatrix::new([[Key::new(KeyCode::________, None); CSIZE]; RSIZE]),
            state: keymap,
            callback,
            wait_cycles: 2,
            cycles: 0,
            cur_strobe: 0,
            push_input,
            mod_update,
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
        // let mut str: String<10> = "strobing ".into();
        // let strobe: String<10> = String::from(self.cur_strobe as u32);
        // str.push_str(&strobe).unwrap();
        // self.execute_info(&str)
    }
    pub fn poll(&mut self) {
        self.next_strobe();
        let c = self.cur_strobe;

        let push_codes = |codes: [(KeyCode, StateType); 2]| {
            codes.iter().for_each(|key| {
                if key.0.is_modifier() {
                    (self.mod_update)((key.0, key.1));
                } else {
                    (self.push_input)((key.0, key.1));
                }
            })
        };

        for r in 0..RSIZE {
            let codes = self.state.matrix[r][c].scan(self.rows[r].is_high());
            if self.state.matrix[r][c].state != self.state.matrix[r][c].prevstate {
                push_codes([
                    (codes[0], self.state.matrix[r][c].state),
                    (codes[1], self.state.matrix[r][c].state),
                ]);
                self.execute_callback(
                    r + 1,
                    c + 1,
                    self.state.matrix[r][c].state,
                    self.state.matrix[r][c].prevstate,
                    codes,
                );
            } else if self.state.matrix[r][c].state == StateType::Hold {
                push_codes([
                    (codes[0], self.state.matrix[r][c].state),
                    (codes[1], self.state.matrix[r][c].state),
                ]);
            }
        }
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
    pub fn dequeue(&mut self, key: KeyCode) {
        self.keys.iter_mut().for_each(|k| {
            if *k == Some(key) {
                *k = None;
            }
        });
    }

    /// push a key into the queue
    /// returns false if the queue is full
    /// returns false if the key is already in the queue
    /// returns true if the key is not in the queue
    pub fn enqueue(&mut self, key: KeyCode) -> bool {
        if self.len() >= QSIZE {
            return false;
        }
        if self.keys.iter().any(|k| *k == Some(key)) {
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

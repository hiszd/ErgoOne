#![allow(dead_code)]

// use rp2040_hal::gpio::PinMode::{Input, Output}

use defmt::{info, println};
use embedded_hal::digital::v2::{InputPin, OutputPin};
use rp2040_hal::gpio::DynPin;

use crate::{
    key::{Default, Key},
    key_codes::KeyCode,
    // mods::mod_tap::ModTap,
};

#[derive(Copy, Clone, PartialEq, PartialOrd)]
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
    callback: fn(row: usize, col: usize, state: StateType, prevstate: StateType),
    push_input: fn(codes: [u8; 6], modifier: u8),
    wait_cycles: u16,
    cycles: u16,
    cur_strobe: usize,
}

impl<const RSIZE: usize, const CSIZE: usize> Matrix<RSIZE, CSIZE> {
    pub fn new(
        rows: [Row; RSIZE],
        cols: [Col; CSIZE],
        callback: fn(row: usize, col: usize, state: StateType, prevstate: StateType),
        push_input: fn(codes: [u8; 6], modifier: u8),
        keymap: KeyMatrix<RSIZE, CSIZE>,
    ) -> Self {
        let mut new = Matrix {
            rows,
            cols,
            state: KeyMatrix::new([[Key::new(KeyCode::________); CSIZE]; RSIZE]),
            callback,
            wait_cycles: 2,
            cycles: 0,
            cur_strobe: 0,
            push_input,
        };
        new.cols[new.cur_strobe].set_high();
        new.clear();
        new
    }
    fn execute_callback(&self, row: usize, col: usize, state: StateType, prevstate: StateType) {
        (self.callback)(row, col, state, prevstate);
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
        for r in 0..RSIZE {
            let st = self.state.matrix[r][c]
                .keystate
                .scan(self.rows[r].is_high());
            if st {
                // println!("{}: {}, {}", st, r, c);
            }
            if self.state.matrix[r][c].keystate.state != self.state.matrix[r][c].keystate.prevstate
            {
                self.execute_callback(
                    r + 1,
                    c + 1,
                    self.state.matrix[r][c].keystate.state,
                    self.state.matrix[r][c].keystate.prevstate,
                );
            }
        }
        // TODO it doesn't make sense to return this at the end of every poll...
        // Some(self.state)
    }
}

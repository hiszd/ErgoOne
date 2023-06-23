#![allow(dead_code)]

use arduino_hal::{
    hal::port::Dynamic,
    port::{
        mode::{Input, Output},
        Pin,
    },
};

use crate::{
    key::{Default, Key},
    key_codes::KeyCode,
    mods::mod_tap::ModTap,
};

#[derive(Copy, Clone, PartialEq, PartialOrd)]
pub enum StateType {
    Tap = 0,
    Hold = 1,
    Idle = 2,
    Off = 3,
}

pub struct Row {
    output: Pin<Output, Dynamic>,
}

impl Row {
    pub fn new(output: Pin<Output, Dynamic>) -> Self {
        Row { output }
    }
    pub fn set_high(&mut self) {
        self.output.set_high()
    }
    pub fn set_low(&mut self) {
        self.output.set_low()
    }
}

pub struct Col {
    input: Pin<Input, Dynamic>,
}

impl Col {
    pub fn new(input: Pin<Input, Dynamic>) -> Self {
        Col { input }
    }
    pub fn is_high(&self) -> bool {
        self.input.is_high()
    }
    pub fn is_low(&self) -> bool {
        self.input.is_low()
    }
    pub fn drain(&mut self) {
        self.input.with_pin_as_output(|p| {
            p.set_low();
        });
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
    info: fn(info: &str),
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
        info: fn(info: &str),
        push_input: fn(codes: [u8; 6], modifier: u8),
        keymap: KeyMatrix<RSIZE, CSIZE>,
    ) -> Self {
        let mut new = Matrix {
            rows,
            cols,
            state: KeyMatrix::new([[Key::new(KeyCode::________); CSIZE]; RSIZE]),
            callback,
            info,
            wait_cycles: 2,
            cycles: 0,
            cur_strobe: 0,
            push_input,
        };
        new.rows[new.cur_strobe].set_high();
        new.clear();
        new
    }
    fn execute_callback(&self, row: usize, col: usize, state: StateType, prevstate: StateType) {
        (self.callback)(row, col, state, prevstate);
    }
    fn execute_info(&self, info: &str) {
        (self.info)(info);
    }
    fn clear(&mut self) {
        for r in self.rows.iter_mut() {
            r.set_low();
        }
    }
    fn next_strobe(&mut self) {
        // Unset current strobe
        self.rows[self.cur_strobe].set_low();

        // Drain stray potential from sense lines
        for c in self.cols.iter_mut() {
            c.drain();
        }

        // Check overflow condition
        if self.cur_strobe >= RSIZE - 1 {
            self.cur_strobe = 0;
        } else {
            // Increment current strobe
            self.cur_strobe += 1;
        }

        // Set new strobe as high
        self.rows[self.cur_strobe].set_high();
        // let mut str: String<10> = "strobing ".into();
        // let strobe: String<10> = String::from(self.cur_strobe as u32);
        // str.push_str(&strobe).unwrap();
        // self.execute_info(&str)
    }
    pub fn poll(&mut self) {
        self.next_strobe();
        let r = self.cur_strobe;
        for c in 0..CSIZE {
            self.state.matrix[r][c]
                .keystate
                .scan(self.cols[c].is_high());
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

// impl<const RSIZE: usize, const CSIZE: usize> From<KeyMatrix<RSIZE, CSIZE>>
//     for usbd_hid::descriptor::KeyboardReport
// {
//     fn from(matrix: KeyMatrix<RSIZE, CSIZE>) -> Self {
//         let mut keycodes = [0u8; 6];
//         let mut keycode_index = 0;
//         let mut modifier = 0;
//
//         let mut push_keycode = |key| {
//             if keycode_index < keycodes.len() {
//                 keycodes[keycode_index] = key;
//                 keycode_index += 1;
//             }
//         };
//
//         for c in matrix.matrix.iter() {
//             for k in c.iter() {
//                 if k.key.state == StateType::Tap {
//                     let keytup: [KeyCode; 2];
//                     (keytup, modifier) = k.tap();
//                     push_keycode(keytup[0] as u8);
//                     push_keycode(keytup[1] as u8);
//                 }
//             }
//         }
//
//         // Scan to generate the correct keycodes given the activated key map
//         // let layer_mapping = key_mapping::NORMAL_LAYER_MAPPING;
//         // for (matrix_column, mapping_column) in matrix.matrix.iter().zip(layer_mapping) {
//         //     for (key_pressed, mapping_row) in matrix_column.iter().zip(mapping_column) {
//         //         if key_pressed.key.state == StateType::Tap
//         //             || key_pressed.key.state == StateType::Hold
//         //         {
//         //             if let Some(bitmask) = mapping_row.modifier_bitmask() {
//         //                 modifier |= bitmask;
//         //             } else {
//         //                 push_keycode(mapping_row as u8);
//         //             }
//         //         }
//         //     }
//         // }
//
//         KeyboardReport {
//             modifier,
//             reserved: 0,
//             leds: 0,
//             keycodes,
//         }
//     }
// }

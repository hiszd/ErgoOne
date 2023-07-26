use crate::keyscanning::StateType;
use crate::Context;
use crate::{key::Key, key_codes::KeyCode};

pub trait Transparent {
    fn tptnew() -> Self
    where
        Self: Sized,
        Self: Transparent;
    fn tpttap(&mut self, ctx: Context) -> [Option<KeyCode>; 4];
    fn tpthold(&mut self, _ctx: Context) -> [Option<KeyCode>; 4];
    fn tptidle(&mut self, _ctx: Context) -> [Option<KeyCode>; 4];
    fn tptoff(&mut self, _ctx: Context) -> [Option<KeyCode>; 4];
    fn get_keys(&mut self, ctx: Context) -> [Option<KeyCode>; 4];
    fn tptscan(&mut self, is_high: bool, ctx: Context) -> [Option<KeyCode>; 4];
}

impl Transparent for Key {
    fn tptnew() -> Self {
        Key {
            cycles: 0,
            raw_state: false,
            cycles_off: 0,
            state: StateType::Off,
            prevstate: StateType::Off,
            keycode: [None; 4],
            previnfo: [false; 6],
            stor: [0; 6],
            typ: "Transparent",
        }
    }
    fn tpttap(&mut self, _ctx: Context) -> [Option<KeyCode>; 4] {
        [None; 4]
    }
    fn tpthold(&mut self, _ctx: Context) -> [Option<KeyCode>; 4] {
        [None; 4]
    }
    fn tptidle(&mut self, _ctx: Context) -> [Option<KeyCode>; 4] {
        [None; 4]
    }
    fn tptoff(&mut self, _ctx: Context) -> [Option<KeyCode>; 4] {
        [None; 4]
    }
    #[doc = " Perform state change as a result of the scan"]
    fn tptscan(&mut self, _is_high: bool, _ctx: Context) -> [Option<KeyCode>; 4] {
        [None; 4]
    }
    fn get_keys(&mut self, ctx: Context) -> [Option<KeyCode>; 4] {
        match self.state {
            StateType::Tap => self.tpttap(ctx),
            StateType::Hold => self.tpthold(ctx),
            StateType::Idle => self.tptidle(ctx),
            StateType::Off => self.tptoff(ctx),
        }
    }
}

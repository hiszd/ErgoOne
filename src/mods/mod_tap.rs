use defmt::error;
use defmt::println;
use defmt::warn;
use heapless::Vec;

use crate::action;
use crate::actions::CallbackActions;
use crate::key::DEBOUNCE_CYCLES;
use crate::key::HOLD_CYCLES;
use crate::keyscanning::StateType;
use crate::Context;
use crate::ARGS;
use crate::{key::Key, key_codes::KeyCode};

pub trait ModTap {
    fn mdtnew(s: &str) -> Self
    where
        Self: Sized,
        Self: ModTap;
    fn mdttap(&mut self, ctx: Context) -> [Option<KeyCode>; 4];
    fn mdthold(&mut self, _ctx: Context) -> [Option<KeyCode>; 4];
    fn mdtidle(&mut self, _ctx: Context) -> [Option<KeyCode>; 4];
    fn mdtoff(&mut self, _ctx: Context) -> [Option<KeyCode>; 4];
    fn get_keys(&mut self, ctx: Context) -> [Option<KeyCode>; 4];
    fn mdtscan(&mut self, is_high: bool, ctx: Context) -> [Option<KeyCode>; 4];
    fn exist_next(&self, ctx: Context, key: KeyCode) -> bool;
}

impl ModTap for Key {
    fn mdtnew(s: &str) -> Self {
        let sr = s.split(",").map(|s| s.trim()).collect::<Vec<&str, 2>>();
        Key {
            cycles: 0,
            raw_state: false,
            cycles_off: 0,
            state: StateType::Off,
            prevstate: StateType::Off,
            keycode: [Some(sr[0].into()), Some(sr[1].into()), None, None],
            previnfo: [false; 6],
            stor: [0; 6],
            typ: "ModTap",
        }
    }
    // when state becomes tap enqueue modifier
    // when state becomes hold never queue key
    fn mdttap(&mut self, ctx: Context) -> [Option<KeyCode>; 4] {
        let [Some(_kc0), Some(kc1), None, None] = self.keycode else {
            return [None; 4];
        };
        // self.previnfo[0] is whether or not a combination was pressed
        self.previnfo[0] = false;
        if kc1.is_modifier() {
            if self.exist_next(ctx, kc1) {
                self.previnfo[0] = true;
            }
        } else {
            error!("{} is not a modifier", kc1);
            return [None; 4];
        }

        match kc1.is_modifier() {
            true => {
                action(CallbackActions::Press, ARGS::KS { code: kc1 });
            }
            false => error!("{} is not a modifier", kc1),
        }
        [Some(kc1), None, None, None]
    }
    fn mdthold(&mut self, _ctx: Context) -> [Option<KeyCode>; 4] {
        let [Some(_kc0), Some(kc1), None, None] = self.keycode else {
            return [None; 4];
        };
        self.previnfo[0] = true;
        match kc1.is_modifier() {
            true => {
                action(CallbackActions::Press, ARGS::KS { code: kc1 });
            }
            false => error!("{} is not a modifier", kc1),
        }
        [Some(kc1), None, None, None]
    }
    fn mdtidle(&mut self, _ctx: Context) -> [Option<KeyCode>; 4] {
        [None; 4]
    }
    // when state goes from tap>off and another key was never pressed enqueue key and pull modifier
    // when state goes from tap>off and another key was pressed never queue key and pull modifier
    // when state goed from hold>off never queue key, but pull modifier
    fn mdtoff(&mut self, ctx: Context) -> [Option<KeyCode>; 4] {
        let [Some(kc0), Some(kc1), None, None] = self.keycode else {
            return [None; 4];
        };
        match self.prevstate {
            StateType::Tap => {
                // if there was not a combination of key pressed during the tap then
                if !self.previnfo[0] && !self.exist_next(ctx, kc1) {
                    println!("no combo");
                    match kc0.is_modifier() {
                        true => error!("{} is a modifier, but shouldn't be", kc0),
                        false => {
                            action(CallbackActions::Release, ARGS::KS { code: kc1 });
                            self.previnfo[4] = true;
                            self.stor[4] = 0;
                        }
                    }
                    return [Some(kc1), None, None, None];
                    // if there was a combination of keys pressed then do nothing
                } else {
                    action(CallbackActions::Release, ARGS::KS { code: kc1 });
                    return [Some(kc1), None, None, None];
                }
            }
            StateType::Hold => {
                match kc0.is_modifier() {
                    true => error!("{} is a modifier, but shouldn't be", kc0),
                    false => {
                        action(CallbackActions::Release, ARGS::KS { code: kc1 });
                    }
                }
                return [Some(kc1), None, None, None];
            }
            StateType::Off => {
                let mut rtrn: [Option<KeyCode>; 4] = [None; 4];
                if self.prevstate == StateType::Off && self.previnfo[4] {
                    if self.stor[4] == 1 {
                        action(CallbackActions::Press, ARGS::KS { code: kc0 });
                        rtrn = [Some(kc0), None, None, None];
                        self.stor[4] += 1;
                    } else if self.stor[4] == 2 {
                        action(CallbackActions::Release, ARGS::KS { code: kc0 });
                        self.previnfo[4] = false;
                        rtrn = [Some(kc0), None, None, None];
                        self.stor[4] += 1;
                    } else {
                        self.stor[4] += 1;
                    }
                }
                rtrn
            }
            _ => {
                return [None; 4];
            }
        }
    }

    /// check to see if another key exists in the queue after the current one
    fn exist_next(&self, ctx: Context, key: KeyCode) -> bool {
        let mut rtrn1 = false;
        // locate key in array
        let ind1: Option<usize> = ctx
            .key_queue
            .iter()
            .position(|k| k.is_some() && k.unwrap() == key);
        let mut srt: usize = 0;
        if ind1.is_some() {
            srt = ind1.unwrap();
        }
        for i in srt..ctx.key_queue.len() {
            if ctx.key_queue[i].is_some() {
                if ctx.key_queue[i].unwrap() != key {
                    rtrn1 = true;
                    warn!("rtrn1 = {}, key = {}", rtrn1, ctx.key_queue[i].unwrap());
                    break;
                }
            }
        }
        if !rtrn1 {
            warn!("rtrn1 = false, key = ''");
        }
        rtrn1
    }
    #[doc = " Perform state change as a result of the scan"]
    fn mdtscan(&mut self, is_high: bool, ctx: Context) -> [Option<KeyCode>; 4] {
        let [Some(kc0), Some(kc1), None, None] = self.keycode else {
            return [None; 4];
        };
        if kc0 == KeyCode::________ && kc1 == KeyCode::________ {
            return [None; 4];
        }
        if is_high {
            if self.cycles < u16::MAX {
                self.cycles += 1;
            }
            self.cycles_off = 0;
        } else {
            if self.cycles_off < u16::MAX {
                self.cycles_off += 1;
            }
            self.cycles = 0;
        }
        self.raw_state = is_high;
        if self.cycles >= DEBOUNCE_CYCLES {
            if self.state == StateType::Tap && self.cycles >= HOLD_CYCLES {
                self.prevstate = self.state;
                self.state = StateType::Hold;
            } else if self.state == StateType::Off || self.state == StateType::Tap {
                self.prevstate = self.state;
                self.state = StateType::Tap;
            } else if self.state == StateType::Hold {
                self.prevstate = self.state;
                self.state = StateType::Hold;
            }
            return self.get_keys(ctx);
        } else if self.cycles_off >= 1 {
            self.prevstate = self.state;
            self.state = StateType::Off;
        }
        self.get_keys(ctx)
    }
    fn get_keys(&mut self, ctx: Context) -> [Option<KeyCode>; 4] {
        match self.state {
            StateType::Tap => self.mdttap(ctx),
            StateType::Hold => self.mdthold(ctx),
            StateType::Idle => self.mdtidle(ctx),
            StateType::Off => self.mdtoff(ctx),
        }
    }
}

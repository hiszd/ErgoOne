use defmt::error;
use defmt::println;
use defmt::warn;

use crate::actions::CallbackActions;
use crate::key::DEBOUNCE_CYCLES;
use crate::key::HOLD_CYCLES;
use crate::keyscanning::StateType;
use crate::Context;
use crate::Operation;
use crate::ARGS;
use crate::{key::Key, key_codes::KeyCode};

pub trait ModTap {
    fn mtnew(KC1: KeyCode, KC2: KeyCode) -> Self
    where
        Self: Sized,
        Self: ModTap;
    fn mttap(
        &mut self,
        ctx: Context,
        action: fn(CallbackActions, ARGS),
    ) -> [Option<(KeyCode, Operation)>; 4];
    fn mthold(
        &mut self,
        _ctx: Context,
        action: fn(CallbackActions, ARGS),
    ) -> [Option<(KeyCode, Operation)>; 4];
    fn mtidle(
        &mut self,
        _ctx: Context,
        action: fn(CallbackActions, ARGS),
    ) -> [Option<(KeyCode, Operation)>; 4];
    fn mtoff(
        &mut self,
        _ctx: Context,
        action: fn(CallbackActions, ARGS),
    ) -> [Option<(KeyCode, Operation)>; 4];
    fn get_keys(
        &mut self,
        ctx: Context,
        action: fn(CallbackActions, ARGS),
    ) -> [Option<(KeyCode, Operation)>; 4];
    fn mtscan(
        &mut self,
        is_high: bool,
        ctx: Context,
        action: fn(CallbackActions, ARGS),
    ) -> [Option<(KeyCode, Operation)>; 4];
    fn exist_next(&self, ks: [Option<KeyCode>; 29], key: KeyCode) -> bool;
}

impl ModTap for Key {
    fn mtnew(KC1: KeyCode, KC2: KeyCode) -> Self {
        Key {
            cycles: 0,
            raw_state: false,
            cycles_off: 0,
            state: StateType::Off,
            prevstate: StateType::Off,
            keycode: [
                Some((KC1, Operation::SendOff)),
                Some((KC2, Operation::SendOn)),
                None,
                None,
            ],
            previnfo: [false; 6],
            stor: [0; 6],
            typ: "ModTap",
        }
    }
    // when state becomes tap enqueue modifier
    // when state becomes hold never queue key
    fn mttap(
        &mut self,
        ctx: Context,
        action: fn(CallbackActions, ARGS),
    ) -> [Option<(KeyCode, Operation)>; 4] {
        let [Some(_kc0), Some(kc1), None, None] = self.keycode else {
            return [None; 4];
        };
        // self.previnfo[0] is whether or not a combination was pressed
        self.previnfo[0] = false;
        if kc1.0.is_modifier() {
            if self.exist_next(ctx.key_queue, kc1.0) {
                self.previnfo[0] = true;
            }
        } else {
            error!("{} is not a modifier", kc1.0);
            return [None; 4];
        }

        match kc1.0.is_modifier() {
            true => {
                action(
                    CallbackActions::Push,
                    ARGS::KS {
                        code: kc1.0,
                        op: kc1.1,
                        st: StateType::Tap,
                    },
                );
            }
            false => error!("{} is not a modifier", kc1.0),
        }
        [Some((kc1.0, kc1.1)), None, None, None]
    }
    fn mthold(
        &mut self,
        _ctx: Context,
        action: fn(CallbackActions, ARGS),
    ) -> [Option<(KeyCode, Operation)>; 4] {
        let [Some(_kc0), Some(kc1), None, None] = self.keycode else {
            return [None; 4];
        };
        self.previnfo[0] = true;
        match kc1.0.is_modifier() {
            true => {
                action(
                    CallbackActions::Push,
                    ARGS::KS {
                        code: kc1.0,
                        op: kc1.1,
                        st: StateType::Hold,
                    },
                );
            }
            false => error!("{} is not a modifier", kc1.0),
        }
        [Some((kc1.0, kc1.1)), None, None, None]
    }
    fn mtidle(
        &mut self,
        _ctx: Context,
        _action: fn(CallbackActions, ARGS),
    ) -> [Option<(KeyCode, Operation)>; 4] {
        [None; 4]
    }
    // when state goes from tap>off and another key was never pressed enqueue key and pull modifier
    // when state goes from tap>off and another key was pressed never queue key and pull modifier
    // when state goed from hold>off never queue key, but pull modifier
    fn mtoff(
        &mut self,
        ctx: Context,
        action: fn(CallbackActions, ARGS),
    ) -> [Option<(KeyCode, Operation)>; 4] {
        match self.prevstate {
            StateType::Tap => {
                let [Some(kc0), Some(kc1), None, None] = self.keycode else {
                    return [None; 4];
                };
                // if there was not a combination of key pressed during the tap then
                if !self.previnfo[0] && !self.exist_next(ctx.key_queue, kc1.0) {
                    println!("no combo");
                    match kc0.0.is_modifier() {
                        true => error!("{} is a modifier, but shouldn't be", kc0.0),
                        false => {
                            action(
                                CallbackActions::Push,
                                ARGS::KS {
                                    code: kc1.0,
                                    op: kc1.1,
                                    st: StateType::Off,
                                },
                            );
                            action(
                                CallbackActions::Push,
                                ARGS::KS {
                                    code: kc0.0,
                                    op: Operation::SendOff,
                                    st: StateType::Tap,
                                },
                            );
                        }
                    }
                    return [Some((kc0.0, kc0.1)), None, None, None];
                    // if there was a combination of keys pressed then do nothing
                } else {
                    action(
                        CallbackActions::Push,
                        ARGS::KS {
                            code: kc1.0,
                            op: kc1.1,
                            st: StateType::Off,
                        },
                    );
                    return [None; 4];
                }
            }
            StateType::Hold => {
                let [Some(kc0), Some(kc1), None, None] = self.keycode else {
                    return [None; 4];
                };
                match kc0.0.is_modifier() {
                    true => error!("{} is a modifier, but shouldn't be", kc0.0),
                    false => {
                        action(
                            CallbackActions::Push,
                            ARGS::KS {
                                code: kc1.0,
                                op: kc1.1,
                                st: StateType::Off,
                            },
                        );
                    }
                }
                return [Some((kc1.0, kc1.1)), None, None, None];
            }
            StateType::Off => {
                return [None; 4];
            }
            _ => {
                return [None; 4];
            }
        }
    }

    /// check to see if another key exists in the queue after the current one
    fn exist_next(&self, ks: [Option<KeyCode>; 29], key: KeyCode) -> bool {
        let mut rtrn1 = false;
        // locate key in array
        let ind1: Option<usize> = ks.iter().position(|k| k.is_some() && k.unwrap() == key);
        let mut srt: usize = 0;
        if ind1.is_some() {
            srt = ind1.unwrap();
        }
        for i in srt..ks.len() {
            if ks[i].is_some() {
                rtrn1 = true;
            }
        }
        warn!("rtrn1 = {}", rtrn1);
        rtrn1
    }
    #[doc = " Perform state change as a result of the scan"]
    fn mtscan(
        &mut self,
        is_high: bool,
        ctx: Context,
        action: fn(CallbackActions, ARGS),
    ) -> [Option<(KeyCode, Operation)>; 4] {
        let [Some(kc0), Some(kc1), None, None] = self.keycode else {
            return [None; 4];
        };
        if kc0.0 == KeyCode::________ && kc1.0 == KeyCode::________ {
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
            }
            return self.get_keys(ctx, action);
        } else if self.cycles_off >= 1 {
            self.prevstate = self.state;
            self.state = StateType::Off;
        }
        self.get_keys(ctx, action)
    }
    fn get_keys(
        &mut self,
        ctx: Context,
        action: fn(CallbackActions, ARGS),
    ) -> [Option<(KeyCode, Operation)>; 4] {
        match self.state {
            StateType::Tap => self.mttap(ctx, action),
            StateType::Hold => self.mthold(ctx, action),
            StateType::Idle => self.mtidle(ctx, action),
            StateType::Off => self.mtoff(ctx, action),
        }
    }
}

use defmt::error;
use defmt::println;
use defmt::warn;

use crate::action;
use crate::actions::CallbackActions;
use crate::key::DEBOUNCE_CYCLES;
use crate::key::HOLD_CYCLES;
use crate::keyscanning::StateType;
use crate::Context;
use crate::Operation;
use crate::ARGS;
use crate::{key::Key, key_codes::KeyCode};

pub trait TapCom {
    fn tcnew(KC1: KeyCode, KCA: (KeyCode, KeyCode)) -> Self
    where
        Self: Sized,
        Self: TapCom;
    fn tctap(&mut self, ctx: Context) -> [Option<(KeyCode, Operation)>; 4];
    fn tchold(&mut self, _ctx: Context) -> [Option<(KeyCode, Operation)>; 4];
    fn tcidle(&mut self, _ctx: Context) -> [Option<(KeyCode, Operation)>; 4];
    fn tcoff(&mut self, _ctx: Context) -> [Option<(KeyCode, Operation)>; 4];
    fn get_keys(&mut self, ctx: Context) -> [Option<(KeyCode, Operation)>; 4];
    fn tcscan(&mut self, is_high: bool, ctx: Context) -> [Option<(KeyCode, Operation)>; 4];
    fn exist_next(&self, ctx: Context, key: KeyCode) -> bool;
}

impl TapCom for Key {
    fn tcnew(KC0: KeyCode, KCA: (KeyCode, KeyCode)) -> Self {
        let (KC1, KC2) = KCA;
        Key {
            cycles: 0,
            raw_state: false,
            cycles_off: 0,
            state: StateType::Off,
            prevstate: StateType::Off,
            keycode: [
                Some((KC0, Operation::SendOn)),
                Some((KC1, Operation::SendOn)),
                Some((KC2, Operation::SendOn)),
                None,
            ],
            previnfo: [false; 6],
            stor: [0; 6],
            typ: "TapCom",
        }
    }
    // when state becomes tap enqueue modifier
    // when state becomes hold never queue key
    fn tctap(&mut self, ctx: Context) -> [Option<(KeyCode, Operation)>; 4] {
        let [Some(kc0), Some(_kc1), Some(_kc2), None] = self.keycode else {
            return [None; 4];
        };
        // DEPRECATED
        // self.stor[0] is the amount of scans that there has been a combo
        // self.stor[1] is the amount of scans that there has NOT been a combo
        if !self.previnfo[0] {
            if kc0.0.is_modifier() {
                // if there is another key pressed
                if self.exist_next(ctx, kc0.0) {
                    self.previnfo[0] = true;
                }
            } else {
                error!("{} is not a modifier", kc0.0);
                return [None; 4];
            }
        }

        if self.prevstate == StateType::Off {
            action(
                CallbackActions::Press,
                ARGS::KS {
                    code: kc0.0,
                    op: kc0.1,
                },
            );
            return [Some((kc0.0, kc0.1)), None, None, None];
        }
        [None; 4]
    }
    fn tchold(&mut self, _ctx: Context) -> [Option<(KeyCode, Operation)>; 4] {
        let [Some(kc0), Some(_kc1), Some(_kc2), None] = self.keycode else {
            return [None; 4];
        };
        self.previnfo[0] = true;
        match kc0.0.is_modifier() {
            true => {
                action(
                    CallbackActions::Press,
                    ARGS::KS {
                        code: kc0.0,
                        op: kc0.1,
                    },
                );
            }
            false => error!("{} is not a modifier", kc0.0),
        }
        [Some((kc0.0, kc0.1)), None, None, None]
    }
    fn tcidle(&mut self, _ctx: Context) -> [Option<(KeyCode, Operation)>; 4] {
        [None; 4]
    }
    // when state goes from tap>off and another key was never pressed enqueue key and pull modifier
    // when state goes from tap>off and another key was pressed never queue key and pull modifier
    // when state goed from hold>off never queue key, but pull modifier
    fn tcoff(&mut self, ctx: Context) -> [Option<(KeyCode, Operation)>; 4] {
        let [Some(kc0), Some(kc1), Some(kc2), None] = self.keycode else {
            return [None; 4];
        };
        match self.prevstate {
            StateType::Tap => {
                // if there was not a combination of key pressed during the tap then
                if !self.previnfo[0] && !self.exist_next(ctx, kc0.0) {
                    println!("no combo");
                    self.previnfo[1] = true;
                    self.stor[4] = 0;
                    action(
                        CallbackActions::Release,
                        ARGS::KS {
                            code: kc0.0,
                            op: kc0.1,
                        },
                    );
                    action(
                        CallbackActions::Press,
                        ARGS::KS {
                            code: kc1.0,
                            op: Operation::SendOn,
                        },
                    );
                    action(
                        CallbackActions::Press,
                        ARGS::KS {
                            code: kc2.0,
                            op: Operation::SendOn,
                        },
                    );
                    return [
                        Some((kc0.0, kc0.1)),
                        Some((kc1.0, kc1.1)),
                        Some((kc2.0, kc2.1)),
                        None,
                    ];
                    // if there was a combination of keys pressed then do nothing
                } else {
                    println!("{}", ctx.key_queue);
                    println!("combo");
                    action(
                        CallbackActions::Release,
                        ARGS::KS {
                            code: kc1.0,
                            op: kc1.1,
                        },
                    );
                    self.previnfo[0] = false;
                    return [Some((kc1.0, kc1.1)), None, None, None];
                }
            }
            StateType::Hold => {
                self.previnfo[1] = false;
                action(
                    CallbackActions::Release,
                    ARGS::KS {
                        code: kc0.0,
                        op: kc0.1,
                    },
                );
                return [Some((kc0.0, kc0.1)), None, None, None];
            }
            StateType::Off => {
                if self.previnfo[1] {
                    if self.stor[4] == 1 {
                        action(
                            CallbackActions::Release,
                            ARGS::KS {
                                code: kc1.0,
                                op: Operation::SendOn,
                            },
                        );
                        action(
                            CallbackActions::Release,
                            ARGS::KS {
                                code: kc2.0,
                                op: Operation::SendOn,
                            },
                        );
                        self.previnfo[1] = false;
                        self.stor[4] += 1;
                    } else {
                        self.stor[4] += 1;
                    }
                }
                return [None; 4];
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
    fn tcscan(&mut self, is_high: bool, ctx: Context) -> [Option<(KeyCode, Operation)>; 4] {
        if self.keycode[0].is_none() && self.keycode[1].is_none() {
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
    fn get_keys(&mut self, ctx: Context) -> [Option<(KeyCode, Operation)>; 4] {
        match self.state {
            StateType::Tap => self.tctap(ctx),
            StateType::Hold => self.tchold(ctx),
            StateType::Idle => self.tcidle(ctx),
            StateType::Off => self.tcoff(ctx),
        }
    }
}

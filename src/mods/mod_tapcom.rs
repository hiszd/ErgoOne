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

pub trait TapCom {
    fn tcnew(KC1: KeyCode, KCA: (KeyCode, KeyCode)) -> Self
    where
        Self: Sized,
        Self: TapCom;
    fn tctap(
        &mut self,
        ctx: Context,
        action: fn(CallbackActions, ARGS),
    ) -> [Option<(KeyCode, Operation)>; 4];
    fn tchold(
        &mut self,
        _ctx: Context,
        action: fn(CallbackActions, ARGS),
    ) -> [Option<(KeyCode, Operation)>; 4];
    fn tcidle(
        &mut self,
        _ctx: Context,
        action: fn(CallbackActions, ARGS),
    ) -> [Option<(KeyCode, Operation)>; 4];
    fn tcoff(
        &mut self,
        _ctx: Context,
        action: fn(CallbackActions, ARGS),
    ) -> [Option<(KeyCode, Operation)>; 4];
    fn get_keys(
        &mut self,
        ctx: Context,
        action: fn(CallbackActions, ARGS),
    ) -> [Option<(KeyCode, Operation)>; 4];
    fn tcscan(
        &mut self,
        is_high: bool,
        ctx: Context,
        action: fn(CallbackActions, ARGS),
    ) -> [Option<(KeyCode, Operation)>; 4];
    fn exist_next(&self, ms: [Option<KeyCode>; 6], ks: [Option<KeyCode>; 6], key: KeyCode) -> bool;
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
                Some((KC1, Operation::SendOff)),
                Some((KC2, Operation::SendOff)),
                None,
            ],
            previnfo: [false; 6],
            stor: [0; 6],
            typ: "TapCom",
        }
    }
    // when state becomes tap enqueue modifier
    // when state becomes hold never queue key
    fn tctap(
        &mut self,
        ctx: Context,
        action: fn(CallbackActions, ARGS),
    ) -> [Option<(KeyCode, Operation)>; 4] {
        let [Some(kc0), Some(_kc1), Some(_kc2), None] = self.keycode else {
            return [None; 4];
        };
        let mut combo: bool = false;
        // self.stor[0] is the amount of scans that there has been a combo
        // self.stor[1] is the amount of scans that there has not been a combo
        if kc0.0.is_modifier() {
            if self.exist_next(ctx.modifiers, ctx.key_queue, kc0.0) {
                if self.stor[0] <= u8::MAX {
                    self.stor[0] += 1;
                }
                if self.stor[0] > 0 {
                    combo = true;
                    self.stor[1] = 0;
                }
            } else {
                if self.stor[1] <= u8::MAX {
                    self.stor[1] += 1;
                }
                if self.stor[1] > 1 as u8 {
                    self.stor[0] = 0;
                }
            }
        } else {
            error!("{} is not a modifier", kc0.0);
            return [None; 4];
        }

        self.previnfo[0] = combo;

        action(
            CallbackActions::mPush,
            ARGS::KS {
                code: Some(kc0.0),
                op: Some(kc0.1),
            },
        );

        [Some((kc0.0, kc0.1)), None, None, None]
    }
    fn tchold(
        &mut self,
        _ctx: Context,
        action: fn(CallbackActions, ARGS),
    ) -> [Option<(KeyCode, Operation)>; 4] {
        let [Some(kc0), Some(_kc1), Some(_kc2), None] = self.keycode else {
            return [None; 4];
        };
        self.previnfo[0] = true;
        match kc0.0.is_modifier() {
            true => {
                action(
                    CallbackActions::mPush,
                    ARGS::KS {
                        code: Some(kc0.0),
                        op: Some(kc0.1),
                    },
                );
            }
            false => error!("{} is not a modifier", kc0.0),
        }
        [Some((kc0.0, kc0.1)), None, None, None]
    }
    fn tcidle(
        &mut self,
        _ctx: Context,
        _action: fn(CallbackActions, ARGS),
    ) -> [Option<(KeyCode, Operation)>; 4] {
        [None; 4]
    }
    // when state goes from tap>off and another key was never pressed enqueue key and pull modifier
    // when state goes from tap>off and another key was pressed never queue key and pull modifier
    // when state goed from hold>off never queue key, but pull modifier
    fn tcoff(
        &mut self,
        ctx: Context,
        action: fn(CallbackActions, ARGS),
    ) -> [Option<(KeyCode, Operation)>; 4] {
        let [Some(kc0), Some(kc1), Some(kc2), None] = self.keycode else {
            return [None; 4];
        };
        match self.prevstate {
            StateType::Tap => {
                // if there was not a combination of key pressed during the tap then
                if !self.previnfo[0] && !self.exist_next(ctx.modifiers, ctx.key_queue, kc0.0) {
                    println!("no combo");
                    self.previnfo[1] = true;
                    action(
                        CallbackActions::mPull,
                        ARGS::KS {
                            code: Some(kc0.0),
                            op: Some(kc0.1),
                        },
                    );
                    match kc1.0.is_modifier() {
                        true => {
                            action(
                                CallbackActions::mPush,
                                ARGS::KS {
                                    code: Some(kc1.0),
                                    op: Some(Operation::SendOff),
                                },
                            );
                        }
                        false => {
                            action(
                                CallbackActions::iPush,
                                ARGS::KS {
                                    code: Some(kc1.0),
                                    op: Some(Operation::SendOff),
                                },
                            );
                        }
                    }
                    match kc2.0.is_modifier() {
                        true => {
                            action(
                                CallbackActions::mPush,
                                ARGS::KS {
                                    code: Some(kc2.0),
                                    op: Some(Operation::SendOff),
                                },
                            );
                        }
                        false => {
                            action(
                                CallbackActions::iPush,
                                ARGS::KS {
                                    code: Some(kc2.0),
                                    op: Some(Operation::SendOn),
                                },
                            );
                        }
                    }
                    return [
                        Some((kc0.0, kc0.1)),
                        Some((kc1.0, kc1.1)),
                        Some((kc2.0, kc2.1)),
                        None,
                    ];
                    // if there was a combination of keys pressed then do nothing
                } else {
                    println!("combo");
                    action(
                        CallbackActions::mPull,
                        ARGS::KS {
                            code: Some(kc1.0),
                            op: Some(kc1.1),
                        },
                    );
                    return [Some((kc1.0, kc1.1)), None, None, None];
                }
            }
            StateType::Hold => {
                self.previnfo[1] = false;
                action(
                    CallbackActions::mPull,
                    ARGS::KS {
                        code: Some(kc0.0),
                        op: Some(kc0.1),
                    },
                );
                return [Some((kc0.0, kc0.1)), None, None, None];
            }
            StateType::Off => {
                if self.previnfo[1] {
                    match kc1.0.is_modifier() {
                        true => {
                            action(
                                CallbackActions::mPull,
                                ARGS::KS {
                                    code: Some(kc1.0),
                                    op: Some(Operation::SendOff),
                                },
                            );
                        }
                        false => {
                            action(
                                CallbackActions::iPull,
                                ARGS::KS {
                                    code: Some(kc1.0),
                                    op: Some(Operation::SendOn),
                                },
                            );
                        }
                    }
                    match kc2.0.is_modifier() {
                        true => {
                            action(
                                CallbackActions::mPull,
                                ARGS::KS {
                                    code: Some(kc2.0),
                                    op: Some(Operation::SendOff),
                                },
                            );
                        }
                        false => {
                            action(
                                CallbackActions::iPull,
                                ARGS::KS {
                                    code: Some(kc2.0),
                                    op: Some(Operation::SendOn),
                                },
                            );
                        }
                    }
                    self.previnfo[1] = false;
                }
                return [None; 4];
            }
            _ => {
                return [None; 4];
            }
        }
    }

    /// check to see if another key exists in the queue after the current one
    fn exist_next(&self, ms: [Option<KeyCode>; 6], ks: [Option<KeyCode>; 6], key: KeyCode) -> bool {
        let mut rtrn1 = false;
        let mut rtrn2 = false;
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
        srt = 0;
        let ind2: Option<usize> = ms.iter().position(|k| k.is_some() && k.unwrap() == key);
        if ind2.is_some() {
            srt = ind2.unwrap() + 1;
        }
        for i in srt..ms.len() {
            if ms[i].is_some() {
                rtrn2 = true;
            }
        }
        warn!("rtrn1 = {}, rtrn2 = {}", rtrn1, rtrn2);
        rtrn1 || rtrn2
    }
    #[doc = " Perform state change as a result of the scan"]
    fn tcscan(
        &mut self,
        is_high: bool,
        ctx: Context,
        action: fn(CallbackActions, ARGS),
    ) -> [Option<(KeyCode, Operation)>; 4] {
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
            StateType::Tap => self.tctap(ctx, action),
            StateType::Hold => self.tchold(ctx, action),
            StateType::Idle => self.tcidle(ctx, action),
            StateType::Off => self.tcoff(ctx, action),
        }
    }
}
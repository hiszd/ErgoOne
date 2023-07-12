// create macro to implement Key for a new trait
#[macro_export]
macro_rules! KeyImpl {
    ($a:expr) => {
        /// Perform state change as a result of the scan
        fn scan(
            &mut self,
            is_high: bool,
            ctx: Context,
            action: fn(CallbackActions, ARGS),
        ) -> [Option<(KeyCode, Operation)>; 4] {
            // println!("{}", is_high);
            // if they KeyCode is empty then don't bother processing
            if self.keycode[0].is_none() && self.keycode[1].is_none() {
                return [None; 4];
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
                return self.get_keys(ctx, action);
            // } else if self.cycles_off >= DEBOUNCE_CYCLES.into() {
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
            // info!("{:?}", self.state);
            // Match all types of self.state
            match self.state {
                StateType::Tap => self.tap(ctx, action),
                StateType::Hold => self.hold(ctx, action),
                StateType::Idle => self.idle(ctx, action),
                StateType::Off => self.off(ctx, action),
            }
        }
    };
}

#![allow(dead_code)]
pub mod reader;

pub mod activation_log {
    use rtic_monotonics::systick::prelude::*;
    use crate::auxiliary::modulo::modulo::Mod;

    systick_monotonic!(Mono, 1000);

    type Time = <Mono as rtic_monotonics::Monotonic>::Instant;
    
    pub struct ActivationLog {
        counter : Mod,
        time : Time,
    }

    impl ActivationLog {
        pub fn new() -> Self {
            Self {
                counter: Mod::new(100),
                time: Mono::now()
            }
        }

        pub fn write(&mut self) -> () {
            self.counter.increment();
            self.time = Mono::now();
        }

        pub fn read(&self) -> (Mod, Time) {
            (self.counter.clone(), self.time)
        }

    }
}

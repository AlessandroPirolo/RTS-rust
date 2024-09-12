#![allow(dead_code)]
pub mod reader;

pub mod activation_log {
    use crate::auxiliary::modulo::modulo::Mod;
    use crate::activation_manager::activation_manager::{Mono, Time};
    use rtic_monotonics::Monotonic;
    
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
            (self.counter.clone(), self.time.clone())
        }

    }
}

pub mod activation_manager {
    /* Here we setup time related things*/
    use rtic_monotonics::systick::prelude::systick_monotonic;
    use rtic_monotonics::systick::prelude::Monotonic;
    use rtic_monotonics::systick::prelude::ExtU32;
    use core::cmp::Ordering;

    systick_monotonic!(Mono, 1000);

    pub type Time = <Mono as rtic_monotonics::Monotonic>::Instant;
    pub type MyDuration = <Mono as rtic_monotonics::Monotonic>::Duration;
    
    const RELATIVE_OFFSET : u32 = 100; 

    pub struct ActivationManager {
        activation_time : Time,
    }

    pub fn get_deadline(starting: Time, deadline : MyDuration) -> Time {
        starting.checked_add_duration(deadline).unwrap()
    }

    pub fn check_deadline(deadline : Time, who : &str) -> () {
        let finish : Time = Mono::now();
        let miss = finish.cmp(&deadline);
        if miss == Ordering::Greater {
            defmt::info!("Deadline of {:?} misses!", who);
        }
    }

    pub fn delay_time() -> Time {
        let duration : MyDuration = 5.millis();
        Mono::now().checked_add_duration(duration).unwrap()
    }

    impl ActivationManager {
        pub fn new() -> Self {
            let st : Time = Mono::now();
            let tst : MyDuration = RELATIVE_OFFSET.millis();
            let _activation_time : Time = st.checked_add_duration(tst).unwrap();
            Self {
               activation_time: _activation_time,
            }
        }

        pub async fn activation_sporadic(&self) -> Time {
            Mono::delay_until(self.activation_time).await;
            self.activation_time
        }

        pub async fn activation_on_call(&self) -> Time {
            let dur : MyDuration = 1000.millis();
            let del : Time = Mono::now().checked_add_duration(dur).unwrap();
            Mono::delay_until(del).await;
            del
        }

        pub async fn activation_cyclic(&self) -> Time {
            Mono::delay_until(self.activation_time).await; 
            self.activation_time
        }
    }
}

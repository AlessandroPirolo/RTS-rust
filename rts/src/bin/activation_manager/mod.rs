pub mod activation_manager {
    use rtic_monotonics::systick::prelude::*;
    systick_monotonic!(Mono, 1000);

    type Time = <Mono as rtic_monotonics::Monotonic>::Instant;
    type MyDuration = <Mono as rtic_monotonics::Monotonic>::Duration;
    
    const RELATIVE_OFFSET : u32 = 100;

    pub struct ActivationManager {
        activation_time : Time,
    }

    impl ActivationManager {
        pub fn new() -> Self {
            let st : Time = Mono::now();
            let tst : MyDuration = RELATIVE_OFFSET.millis();
            Self {
               activation_time: st + tst,
            }
        }

        pub async fn activation_sporadic(&self) -> () {
            Mono::delay_until(self.activation_time).await;
        }

        pub async fn activation_cyclic(&self) -> Time {
            Mono::delay_until(self.activation_time).await; 
            self.activation_time
        }
    }
}

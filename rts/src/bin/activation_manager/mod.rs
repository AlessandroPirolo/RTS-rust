pub mod activation_manager {
    use rtic_monotonics::systick::prelude::*;
    systick_monotonic!(Mono, 100);
    
    const RELATIVE_OFFSET : u32 = 100;
    const START_TIME = Mono::now();
    //const TASK_START_TIME : u32 = RELATIVE_OFFSET.millis();

    pub struct ActivationManager {
        activation_time : Mono, 
    }

    impl ActivationManager {
        pub async fn activation_sporadic(&self) -> () {
            Mono::delay_until(self.activation_time).await;
        }

        pub async fn activation_cyclic(&self, mut next_time : Mono::Ticks) -> () {
            next_time = self.activation_time;
            Mono::delay_until(self.activation_time).await;
        }
    }
}

pub mod timing_event {
    use rtic_sync::arbiter::Arbiter;

    pub struct TimingEvent {
        event: Arbiter<bool>,
    }

    impl TimingEvent {
        pub fn new() -> Self {
            Self { event: Arbiter::new(false) }
        }

        pub async fn set(&self) {
            *self.event.access().await = false;
        } 
        
        pub async fn cancel(&self) {
            *self.event.access().await = true;
        }

        pub async fn check(&self) -> bool {
            *self.event.access().await
        }
    }
}

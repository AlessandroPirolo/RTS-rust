pub mod event_queue{
    use embassy_sync::{
        channel::Sender,
        blocking_mutex::raw::CriticalSectionRawMutex
    };

    pub struct EventQueue{
        barrier : bool
    }

    impl EventQueue {
        pub fn new() -> Self {
            Self{barrier: false}
        }

        pub fn signal(&mut self, sender: Sender<'static, CriticalSectionRawMutex, u32, 1>) -> () {
            self.barrier = true;
            let _ = sender.try_send(1);
        }

        pub fn wait(&mut self) -> () {
            self.barrier = false;    
        }
    }
}

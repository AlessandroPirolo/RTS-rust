pub mod event_queue{
    pub struct EventQueue{
        barrier : bool
    }

    impl EventQueue {
        pub fn new() -> Self {
            Self{barrier: false}
        }

        pub fn signal(&mut self) -> () {
            self.barrier = true;
        }

        pub fn wait(&mut self) -> bool {
            if self.barrier {
                self.barrier = false;
                true
            } else {
                false
            }
        }
    }
}

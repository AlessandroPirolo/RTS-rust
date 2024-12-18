pub mod act_log_reader {
    use rtic_sync::arbiter::Arbiter;

    pub struct ActLogReader {
        sem: Arbiter<bool>,
    }

    impl ActLogReader {
        pub fn new() -> Self {
            Self {
                sem: Arbiter::new(false),
            }
        }

        pub async fn signal(&self) {
            *self.sem.access().await = true;
        }

        pub async fn wait(&self) -> bool {
            if *self.sem.access().await == true {
                *self.sem.access().await = false;
                true
            } else {
                false
            }
        }
    }
}

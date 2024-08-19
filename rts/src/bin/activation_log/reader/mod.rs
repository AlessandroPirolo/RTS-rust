pub mod act_log_reader {

    pub struct ActLogReader {
        sem : bool
    }

    impl ActLogReader {
        pub fn new() -> Self {
            Self {
                sem: false
            }
        }

        pub fn signal(&mut self) -> () {
            self.sem = true;
        }

        pub fn wait(&self) -> bool {
            self.sem
        }
    }
}

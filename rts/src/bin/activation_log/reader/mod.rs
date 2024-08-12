pub mod act_log_reader {

    use crate::activation_log::suspension_obj::suspension_obj::SuspensionObj;

    pub struct ActLogReader {
        sem : SuspensionObj
    }

    impl ActLogReader {
        pub fn new() -> Self {
            Self {
                sem: SuspensionObj::new()
            }
        }

        pub fn signal(&mut self) -> () {
            self.sem.send();
        }

        pub fn wait(&mut self) -> () {
            self.sem.receive();
        }
    }
}

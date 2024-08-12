pub mod suspension_obj {

    use rtic_sync::channel::*;
    use rtic_sync::make_channel;

    pub struct SuspensionObj {
        receiver : Receiver<'static, u32, 1>,
        sender : Sender<'static, u32, 1>
    }

    impl SuspensionObj {
        pub fn new() -> Self {
            let (s, r) = make_channel!(u32, 1);
            Self {
                receiver : r,
                sender : s
            }
        }

        pub async fn send(&mut self) -> () {
            self.sender.send(1).await;
        }

        pub async fn receive(&mut self) -> () {
            self.receiver.recv().await;  
        }
    }
}

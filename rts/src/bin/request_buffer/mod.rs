pub mod request_buffer {
     use crate::parameters::parameters::request_buffer::REQUEST_BUFFER_RANGE;
     use crate::auxiliary::modulo::modulo::Mod;
     use embassy_sync::{
         channel::Sender,
         blocking_mutex::raw::CriticalSectionRawMutex};
     

     pub struct RequestBuffer {
         buffer : [i32; REQUEST_BUFFER_RANGE as usize],
         insert_index : Mod,
         extract_index : Mod,
         current_size : u32,
         //barrier : bool,
     }

     impl RequestBuffer {
         pub fn new() -> Self {
             Self {
                 insert_index: Mod::new(REQUEST_BUFFER_RANGE),
                 extract_index: Mod::new(REQUEST_BUFFER_RANGE),
                 current_size: 0,
                 //barrier: false,
                 buffer: [0; REQUEST_BUFFER_RANGE as usize],
             }
         }

         pub fn deposit(&mut self, activation_parameter : i32, sender: Sender<'static, CriticalSectionRawMutex, u32, 1>) -> bool {
            if self.current_size < REQUEST_BUFFER_RANGE {
                self.buffer[self.insert_index.to_int() as usize] = activation_parameter;
                self.insert_index.increment();
                self.current_size += 1;
                let _ = sender.try_send(1);
                true
            } else {
                false
            }   
         }

         pub fn extract(&mut self) -> i32 {
            let result : i32 = self.buffer[self.extract_index.to_int() as usize];
            self.extract_index.increment();
            self.current_size -= 1;
            //self.barrier = self.current_size != 0;
            result
         }
     }


}

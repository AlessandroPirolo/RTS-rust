pub mod request_buffer {
     use crate::parameters::parameters::request_buffer::REQUEST_BUFFER_RANGE;
     use crate::auxiliary::modulo::modulo;

     pub struct RequestBuffer {
         buffer : [i32; REQUEST_BUFFER_RANGE as usize],
         insert_index : modulo::Mod,
         extract_index : modulo::Mod,
         current_size : u32,
         barrier : bool,
     }

     impl RequestBuffer {
         pub fn new() -> Self {
             Self {
                 insert_index: modulo::new(REQUEST_BUFFER_RANGE),
                 extract_index: modulo::new(REQUEST_BUFFER_RANGE),
                 current_size: 0,
                 barrier: false,
                 buffer: [0; REQUEST_BUFFER_RANGE as usize],
             }
         }

         pub fn deposit(&mut self, activation_parameter : i32) -> bool {
            if self.current_size < REQUEST_BUFFER_RANGE {
                self.buffer[self.insert_index.to_int() as usize] = activation_parameter;
                self.insert_index.increment();
                self.current_size += 1;
                self.barrier = true;
                true
            } else {
                false
            }   
         }

         pub fn extract(&mut self) -> i32 {
             let result : i32 = self.buffer[self.extract_index.to_int() as usize];
             self.extract_index.increment();
             self.current_size -= 1;
             self.barrier = self.current_size != 0;
             result
         }
     }


}

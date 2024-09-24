pub mod parameters {

    // Regular producer paramters
    pub mod regular {
        use crate::activation_manager::activation_manager::MyDuration;
        use rtic_monotonics::systick::prelude::ExtU32;
        
        const PERIOD: u32 = 1000;
        const DEADLINE: u32 = 800; //500

        pub const REGULAR_PRODUCER_WORKLOAD: i32 = 756;
        pub const ON_CALL_PRODUCER_WORKLOAD: i32 = 278;

    pub const ACTIVATION_CONDITION: u32 = 2;

        pub fn get_period() -> MyDuration {
            PERIOD.millis()
        }
        
        pub fn get_deadline() -> MyDuration {
            DEADLINE.millis()
        }
    }

    pub mod act_log_reader {
        use crate::activation_manager::activation_manager::MyDuration;
        use rtic_monotonics::systick::prelude::ExtU32;
        
        pub const LOAD: i32 = 139;
        const INTER_ARRIVAL_TIME: u32 = 3000;
        const DEADLINE: u32 = 1000;

        pub fn get_deadline() -> MyDuration {
            DEADLINE.millis()
        }

        pub fn get_inter_arrival_time() -> MyDuration {
            INTER_ARRIVAL_TIME.millis()
        }
    }

    pub mod request_buffer {
        pub const REQUEST_BUFFER_RANGE: u32 = 5;
    }

    pub mod on_call_prod {
        use crate::activation_manager::activation_manager::MyDuration;
        use rtic_monotonics::systick::prelude::ExtU32;

        pub const DEADLINE: u32 = 800;
        const INTER_ARRIVAL_TIME: u32 = 3000;

        pub fn get_deadline() -> MyDuration {
            DEADLINE.millis()
        }
        
        pub fn get_inter_arrival_time() -> MyDuration {
            INTER_ARRIVAL_TIME.millis()
        }
    }

    pub mod ext_event_serv {
        use crate::activation_manager::activation_manager::MyDuration;
        use rtic_monotonics::systick::prelude::ExtU32;
        
        pub const DEADLINE: u32 = 100;
        const INTER_ARRIVAL_TIME: u32 = 5000;

        pub fn get_deadline() -> MyDuration {
            DEADLINE.millis()
        }
        
        pub fn get_inter_arrival_time() -> MyDuration {
            INTER_ARRIVAL_TIME.millis()
        }
    }

    pub mod force_inter {
        use crate::activation_manager::activation_manager::MyDuration;
        use rtic_monotonics::systick::prelude::ExtU32;
        
        pub const DEADLINE: u32 = 100;
        pub const PERIOD: u32 = 5000;

        pub fn get_deadline() -> MyDuration {
            DEADLINE.millis()
        }

        pub fn get_period() -> MyDuration {
            PERIOD.millis()
        }
    }
}

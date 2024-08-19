pub mod parameters {

    // Regular producer paramters
    pub mod regular {
        pub const PERIOD: u32 = 1000; // in millisec

        pub const REGULAR_PRODUCER_WORKLOAD: i32 = 756;
        pub const ON_CALL_PRODUCER_WORKLOAD: i32 = 278;

        pub const ACTIVATION_CONDITION: u32 = 2;
    }

    pub mod activation_log_reader {
        pub const LOAD: i32 = 139;
    }

    pub mod request_buffer {
        pub const REQUEST_BUFFER_RANGE: u32 = 5;
    }
}

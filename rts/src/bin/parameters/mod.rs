#![allow(dead_code)]
pub mod parameters {

    // Regular producer paramters
    pub mod regular {

        use crate::production_workload::production_workload;
        use crate::auxiliary::auxiliary;

        pub const PRIORITY: u32 = 7;
        pub const PERIOD: u32 = 1000; // in millisec

        const REGULAR_PRODUCER_WORKLOAD: u32 = 756;
        const ON_CALL_PRODUCER_WORKLOAD: u32 = 278;

        const ACTIVATION_CONDITION: u32 = 2;

        pub fn operation(mut workload: production_workload::WorkloadProd, mut aux: auxiliary::Aux) -> () {
            workload.small_whetstone(REGULAR_PRODUCER_WORKLOAD);

            if aux.due_activation(ACTIVATION_CONDITION) {
                /*if !o.start(ON_CALL_PRODUCER_WORKLOAD) {
                    defmt::info!("Failed sporadic activation.");
                }
            }

            if aux.check_due() {
                log_reader.signal()*/
            }
            defmt::info!("End of cyclic activation.");
        }
    }

    // On call producer parameters
    pub mod on_call {

        use crate::production_workload::production_workload;

        const PRIORITY: u32 = 5;

        pub fn operarion(mut workload : production_workload::WorkloadProd, load: u32) -> () {
            workload.small_whetstone(load);
            defmt::info!("End of sporadic activation.");
        }
    }

    pub mod activation_log_reader {

        use crate::production_workload::production_workload;
        use crate::auxiliary::modulo::modulo::Mod;

        const PRIORITY: u32 = 3;
        const LOAD: u32 = 139;

        pub fn operation(
            mut workload: production_workload::WorkloadProd,
            _interrupt_arrival_counter: Mod, /*interrupt_arrival_time :*/
        ) -> () {
            workload.small_whetstone(LOAD);
            // add activation_log read
        }
    }

    pub mod request_buffer {
        pub const REQUEST_BUFFER_RANGE: u32 = 5;
    }

    pub mod external_event_server {
        pub fn operation() -> () {
            // activation log write
        }
    }
}

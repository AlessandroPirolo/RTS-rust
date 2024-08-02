use production_workload as w;
use auxiliary as a;
use modulo as m;
use on_call_producer as o;
use activation_log_reader as log_reader;

mod parameters {
    
    // Regular producer paramters
    mod regular {
        const PRIORITY: u32 = 7;
        const PERIOD: u32 = 1000; // in millisec
    
        const REGULAR_PRODUCER_WORKLOAD: u32 = 756;
        const ON_CALL_PRODUCER_WORKLOAD: u32 = 278;

        const ACTIVATION_CONDITION : m:Mod(5) = 2;

        pub fn operation(workload : w::WorkloadProd, aux : a::Auxiliary) -> ! {
            workload.small_whetstone(REGULAR_PRODUCER_WORKLOAD);

            if aux.due_activation(ACTIVATION_CONDITION) {
                if !o.start(ON_CALL_PRODUCER_WORKLOAD) {
                    hprint!("Failed sporadic activation.");
                }
            }

            if aux.check_due() {
                log_reader.signal()
            }
            hprint!("End of cyclic activation.")
        }
    }

    // On call producer parameters
    mod on_call  {
        const PRIORITY : u32 = 5;

        pub fn operarion(workload : w::WorkloadProd, load : u32) -> ! {
            worload.small_whetstone(load);
            hprint!("End of sporadic activation.");
        }
    }

    mod activation_log_reader {
        const PRIORITY : u32 = 3;
        const LOAD : u32 = 139;

        pub fn operation(worload : w::WorkloadProd, interrupt_arrival_counter : m::Mod, interrupt_arrival_time : ) -> ! {
            worload.small_whetstone(LOAD);
            // add activation_log read  
        }
    }

    mod request_buffer {
        const REQUEST_BUFFER_RANGE : u32 = 5;
    }

    mod external_event_server {
        pub fn operation() -> ! {
            // activation log write
        }
    }
}

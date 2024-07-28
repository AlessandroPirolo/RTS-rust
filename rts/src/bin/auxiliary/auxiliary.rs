use modulo as m;

mod auxiliary {
    
    const FACTOR : u16 = 3;

    #[derive(Debug)] 
    struct Aux {
        request_counter : m::Mod,
        run_counter : m::Mod
    }

    impl Aux {
        pub fn new() -> Self {
            Self {
                m::new(5),
                m::new(1000)
            }
        }
        
        pub fn due_activation(&self, param : m::Mod) -> bool {
            self.request_counter.increment();
            request_counter == param;
        } 

        pub fn check_due(&self) -> bool {
            self.run_counter.increment();   
            let divisor : u16 = run_counter.to_int / factor;
            (divisor * factor) == run_counter.to_int;

        }
    }
}

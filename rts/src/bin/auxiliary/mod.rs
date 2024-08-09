pub mod modulo;

pub mod auxiliary {

    use crate::auxiliary::modulo::modulo;

    const FACTOR : u16 = 3;

    #[derive(Debug)]
    pub struct Aux {
        request_counter: modulo::Mod,
        run_counter: modulo::Mod,
    }

    impl Aux {
        pub fn new() -> Self {
            Self {
                request_counter: modulo::Mod::new(5),
                run_counter: modulo::Mod::new(1000),
            }
        }

        pub fn due_activation(&mut self, param: u16) -> bool {
            self.request_counter.increment();
            self.request_counter.to_int() == param
        }

        pub fn check_due(&mut self) -> bool {
            self.run_counter.increment();
            let divisor: u16 = self.run_counter.to_int() / FACTOR;
            (divisor * FACTOR) == self.run_counter.to_int()
        }
    }
}

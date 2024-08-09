pub mod modulo {
    #[derive(Debug)]
    pub struct Mod {
        modulus: u16,
        value: u16,
    }

    impl Mod {
        pub fn new(modulus: u16) -> Self {
            Self { modulus, value: 0 }
        }

        pub fn increment(&mut self) -> () {
            let mut tmp: u16 = self.value + 1;
            tmp = ((tmp % self.modulus) + self.modulus) % self.modulus;
            self.value = tmp;
        }

        pub fn to_int(&self) -> u16 {
            self.value
        }
    }
}

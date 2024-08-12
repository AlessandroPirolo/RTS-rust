pub mod modulo {
    #[derive(Debug, Clone)]
    pub struct Mod {
        modulus: u32,
        value: u32,
    }

    impl Mod {
        pub fn new(modulus: u32) -> Self {
            Self { modulus, value: 0 }
        }

        pub fn increment(&mut self) -> () {
            let mut tmp: u32 = self.value + 1;
            tmp = ((tmp % self.modulus) + self.modulus) % self.modulus;
            self.value = tmp;
        }

        pub fn to_int(&self) -> u32 {
            self.value
        }
    }
}

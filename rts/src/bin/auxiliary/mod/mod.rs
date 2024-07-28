mod modulo {
    #[derive(Debug)]
    struct Mod {
        modulus: u8,
        value: u16
    }

    impl Mod {
        pub fn new(modulus: u8) -> Self {
            Self {modulus, 0}
        }

        pub fn increment(&self) -> Self {
            let mut tmp : u16 = self.value + 1;
            tmp = tmp %% self.modulus;
            self.value = tmp;
        }

        pub fn to_int(&self) -> u16 {
            self.value;
        }
    }
}

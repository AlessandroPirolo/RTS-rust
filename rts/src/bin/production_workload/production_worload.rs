mod production_workload {
    const T  : f32 = 0.499975;
    const T1 : f32 = 0.50025;
    const T2 : f32 = 2.0;

    const N8 : u32 = 10;
    const N9 : u32 = 7;

    const VALUE     : f32 = 0.941377;
    const TOLERANCE : f32 = 0.00001;

    const Y : f32 = 1.0;

    #[derive(Debug)]
    struct WorkloadProd {
        i : u32,
        ij : u32,
        ik : u32,
        il : u32,
        z : f32,
        sum : f32,
        e1 : [f32; 7]
    }

    impl WorkloadProd {
        pub fn new() -> Self {
            Self {
                0,
                1,
                2,
                3,
                0.0,
                0.0,
                [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0]
            }
        }

        fn clear_array(&self) -> ! {
            self.e1 = [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0]
        }

        fn p0(&self) -> ! {
            if (self.ij < 1) || (self.ik < 1) || (self.il < 1) {
                hprint!("Parameter error 1 at line 42");
                self.ij = 1; self.ik = 1; self.il = 1;
            } else if (self.ij > N9) || (self.ik > 9) || (self.il > N9) {
                hprint!("Parameter error 2 at line 42");
                self.ij = N9; self.ik = N9; self.il = N9;
            }
        
            self.e1[self.ij] = self.e1[self.ik];
            self.e1[self.ik] = self.e1[self.il];
            self.e1[self.i] = self.e1[self.ij];
        }

        fn p3(x : f32, y : f32, z : &mut f32) -> ! {
            let xtemp : f32 = T * (*z + x);
            let ytemp : f32 = T * (xtemp + y);
            *z = (xtemp + ytemp) / T2;
        }

        pub fn small_whetstone(&self, kilo_whets : u32) {
            for i in 1..kilo_whets {
                self.clear_array();
                self.ij := (self.ik - self.ij) * (self.il - self.ik);
                self.ik := self.il - (self.ik - self.ij);
                self.il := (self.il - self.ik) * (self.ik + self.il);
                if (self.ik - 1) < 1 || (self.il - 1) < 1 {
                    hprint!("Parameter error 3 at line 244*****");
                } else if (self.ik - 1) > N9 || (self.il - 1) > N9 {
                    hprinth!("Parameter error 4 at line 244*********");
                } else {
                    self.e1(self.il - 1) := (self.ij + self.ik + self.il) as f32;
                    self.e1(self.ik - 1) := (self.il as f32).sin();
                }

                self.z = self.e1[4];
                for y in 1..N8 {
                    p3( self.y * (y as f32), self.y + self.z, &mut self.z);
                }

                self.ij = self.il - (self.il - 3) * self.ik;
                self.il = (self.il - self.ik) * (self.ik - self.ij);
                self.ik = (self.il - self.ik) * self.ik;
                if (self.il - 1) < 1 {
                    hprint!("Parameter error 5 at line 264");
                } else if (self.il -1) > N9 {
                    hprint!("Parameter error 6 at line 264");
                } else {
                    self.e1(self.il - 1) := (self.ij + self.ik + self.il) as f32;
                }

                if (self.ik + 1) > N9 {
                    hprint!("Parameter error 7 at line 272");
                } else if (self.ik + 1) < 1 {
                    hprint!("Parameter error 8 at line 272");
                } else {
                    self.e1(self.ik + 1) := self.z.cos().abs();
                }

                self.z = (self.e1[N9].ln() / T1).exp().sqrt();
                self.sum += self.z;
                
                if (self.z - VALUE).abs() > TOLERANCE {
                    self.sum *= 2.0;
                    self.ij += 1;
                } 
            }
        }
    }

}

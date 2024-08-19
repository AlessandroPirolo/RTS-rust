#![allow(dead_code)]
pub mod production_workload {

    use num_traits::real::Real;

    const T: f32 = 0.499975;
    const T1: f32 = 0.50025;
    const T2: f32 = 2.0;

    const N8: u32 = 10;
    const N9: u32 = 7;

    const VALUE: f32 = 0.941377;
    const TOLERANCE: f32 = 0.00001;

    const Y: f32 = 1.0;

    #[derive(Debug)]
    pub struct WorkloadProd {
        i: u32,
        ij: u32,
        ik: u32,
        il: u32,
        z: f32,
        sum: f32,
        e1: [f32; 7],
    }

    impl WorkloadProd {
        pub fn new() -> Self {
            Self {
                i: 0,
                ij: 1,
                ik: 2,
                il: 3,
                z: 0.0,
                sum: 0.0,
                e1: [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0],
            }
        }

        fn clear_array(&mut self) -> () {
            self.e1 = [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0]
        }

        fn p0(&mut self) -> () {
            if (self.ij < 1) || (self.ik < 1) || (self.il < 1) {
                defmt::info!("Parameter error 1 at line 42");
                self.ij = 1;
                self.ik = 1;
                self.il = 1;
            } else if (self.ij > N9) || (self.ik > 9) || (self.il > N9) {
                defmt::info!("Parameter error 2 at line 42");
                self.ij = N9;
                self.ik = N9;
                self.il = N9;
            }

            let ij_idx : usize = self.ij as usize;
            let ik_idx : usize = self.ik as usize;
            let i_idx : usize = self.i as usize;
            let il_idx : usize = self.il as usize;


            self.e1[ij_idx] = self.e1[ik_idx];
            self.e1[ik_idx] = self.e1[il_idx];
            self.e1[i_idx] = self.e1[ij_idx];
        }

        fn p3(x: f32, y: f32, z: &mut f32) -> () {
            let xtemp: f32 = T * (*z + x);
            let ytemp: f32 = T * (xtemp + y);
            *z = (xtemp + ytemp) / T2;
        }

        pub fn small_whetstone(&mut self, kilo_whets: i32) -> () {
            for _i in 1..kilo_whets {
                self.clear_array();
                self.ij = (self.ik - self.ij) * (self.il - self.ik);
                self.ik = self.il - (self.ik - self.ij);
                self.il = (self.il - self.ik) * (self.ik + self.il);
                if (self.ik - 1) < 1 || (self.il - 1) < 1 {
                    defmt::info!("Parameter error 3 at line 244*****");
                } else if (self.ik - 1) > N9 || (self.il - 1) > N9 {
                    defmt::info!("Parameter error 4 at line 244*********");
                } else {
                    let il_idx : usize = (self.il - 1) as usize;
                    let ik_idx : usize = (self.ik - 1) as usize;
                    self.e1[il_idx] = (self.ij + self.ik + self.il) as f32;
                    self.e1[ik_idx] = (self.il as f32).sin();
                }

                self.z = self.e1[4];
                for i in 1..N8 {
                    Self::p3(Y * (i as f32), Y + self.z, &mut self.z);
                }

                self.ij = self.il - (self.il - 3) * self.ik;
                self.il = (self.il - self.ik) * (self.ik - self.ij);
                self.ik = (self.il - self.ik) * self.ik;
                if (self.il - 1) < 1 {
                    defmt::info!("Parameter error 5 at line 264");
                } else if (self.il - 1) > N9 {
                    defmt::info!("Parameter error 6 at line 264");
                } else {
                    let il_idx : usize = (self.il - 1) as usize;
                    self.e1[il_idx] = (self.ij + self.ik + self.il) as f32;
                }

                if (self.ik + 1) > N9 {
                    defmt::info!("Parameter error 7 at line 272");
                } else if (self.ik + 1) < 1 {
                    defmt::info!("Parameter error 8 at line 272");
                } else {
                    let ik_idx : usize = (self.ik + 1) as usize;
                    self.e1[ik_idx] = self.z.cos().abs();
                }

                self.z = (self.e1[N9 as usize].ln() / T1).exp().sqrt();
                self.sum += self.z;

                if (self.z - VALUE).abs() > TOLERANCE {
                    self.sum *= 2.0;
                    self.ij += 1;
                }
            }
        }
    }
}

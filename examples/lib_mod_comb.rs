const MOD: usize = 998244353;

fn main() {
    let mc = mod_comb::ModComb::init(MOD, 10);
    println!("{}", mc.calc(5, 3));
}

mod mod_comb {
    pub struct ModComb {
        modulo: usize,
        fac: Vec<usize>,
        finv: Vec<usize>,
    }

    impl ModComb {
        pub fn init(modulo: usize, max: usize) -> ModComb {
            let mut fac = vec![1; max];
            let mut finv = vec![1; max];
            let mut inv = vec![1; max];
            for i in 2..max {
                fac[i] = fac[i - 1] * i % modulo;
                inv[i] = modulo - inv[modulo % i] * (modulo / i) % modulo;
                finv[i] = finv[i - 1] * inv[i] % modulo;
            }
            ModComb {
                modulo,
                fac,
                finv,
            }
        }

        pub fn calc(&self, n: usize, k: usize) -> usize {
            if n < k {
                return 0;
            }
            self.fac[n] * (self.finv[k] * self.finv[n - k] % self.modulo) % self.modulo
        }
    }
}

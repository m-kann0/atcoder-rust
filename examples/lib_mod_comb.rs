const MOD: usize = 998244353;

struct ModComb {
    fac: Vec<usize>,
    finv: Vec<usize>,
    inv: Vec<usize>,
}

impl ModComb {
    fn init(max: usize) -> ModComb {
        let mut fac = vec![1; max];
        let mut finv = vec![1; max];
        let mut inv = vec![1; max];
        for i in 2..max {
            fac[i] = fac[i - 1] * i % MOD;
            inv[i] = MOD - inv[MOD % i] * (MOD / i) % MOD;
            finv[i] = finv[i - 1] * inv[i] % MOD;
        }
        return ModComb {
            fac: fac,
            finv: finv,
            inv: inv,
        }
    }

    fn calc(&self, n: usize, k: usize) -> usize {
        if n < k {
            return 0;
        }
        return self.fac[n] * (self.finv[k] * self.finv[n - k] % MOD) % MOD;
    }
}

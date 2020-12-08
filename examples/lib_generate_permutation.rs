fn main() {
    let perms = permutation::generate(4);
    for p in perms {
        eprintln!("p = {:?}", p);
    }

    // 10桁はMLEの可能性あり。
    let perms = permutation::generate(10);
    // eprintln!("perms.len() = {:?}", perms.len());
}

mod permutation {
    pub fn generate(n: usize) -> Vec<Vec<usize>> {
        let mut used = vec![false; n];
        let mut perm = vec![0; n];
        let mut result = Vec::new();
        rec(0, n, &mut used, &mut perm, &mut result);
        result
    }

    fn rec(pos: usize, n: usize, used: &mut Vec<bool>, perm: &mut Vec<usize>, result: &mut Vec<Vec<usize>>) {
        if pos == n {
            result.push(perm.clone());
            return
        }

        for i in 0..n {
            if !used[i] {
                perm[pos] = i;
                used[i] = true;
                rec(pos + 1, n, used, perm, result);
                used[i] = false;
            }
        }
    }
}
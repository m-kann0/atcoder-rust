fn main() {
    let perms = generate_permutation(4);
    for p in perms {
        eprintln!("p = {:?}", p);
    }

    let perms = generate_permutation(10);
    eprintln!("perms.len() = {:?}", perms.len());
}

fn generate_permutation(n: usize) -> Vec<Vec<usize>> {
    let mut used = vec![false; n];
    let mut perm = vec![0; n];
    let mut result = Vec::new();
    permutation(0, n, &mut used, &mut perm, &mut result);
    result
}

fn permutation(pos: usize, n: usize, used: &mut Vec<bool>, perm: &mut Vec<usize>, result: &mut Vec<Vec<usize>>) {
    if pos == n {
        result.push(perm.clone());
        return
    }

    for i in 0..n {
        if !used[i] {
            perm[pos] = i;
            used[i] = true;
            permutation(pos + 1, n, used, perm, result);
            used[i] = false;
        }
    }
}
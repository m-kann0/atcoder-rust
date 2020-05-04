use std::collections::HashMap;

fn main() {
    let result = prime_factorization(140);
    println!("{:?}", result);
}

/// 素因数分解
fn prime_factorization(mut n: usize) -> HashMap<usize, usize> {
    let mut result: HashMap<_, _> = HashMap::new();

    for d in 2.. {
        if d * d > n {
            break;
        }
        while n % d == 0 {
            *result.entry(d).or_insert(0) += 1;
            n /= d;
        }
    }

    if n != 1 {
        result.insert(n, 1);
    }

    return result;
}

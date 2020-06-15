fn main() {
    let is_prime = make_sieve_of_eratosthenes(100);
    for i in 0..101 {
        println!("{}: {}", i, is_prime[i])
    }
}

/// エラトステネスの篩により、素数判定配列を生成して返却する
fn make_sieve_of_eratosthenes(max: usize) -> Vec<bool> {
    let mut is_prime = vec![true; max + 1];
    is_prime[0] = false;
    is_prime[1] = false;

    let mut i = 2;
    while i * i <= max {
        if is_prime[i] {
            let mut j = i * 2;
            while j <= max {
                is_prime[j] = false;
                j += i;
            }
        }
        i += 1;
    }

    return is_prime;
}

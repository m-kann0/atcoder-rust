fn main() {
    println!("{:?}", enumerate_divisors(10));
    println!("{:?}", enumerate_divisors(11));
}

/// 約数列挙
fn enumerate_divisors(n: usize) -> Vec<usize> {
    let mut divisors = Vec::new();

    let mut i = 1;
    while i * i <= n {
        if n % i == 0 {
            divisors.push(i);
            if n / i != i {
                divisors.push(n / i);
            }
        }
        i += 1;
    }
    divisors.sort();
    return divisors;
}

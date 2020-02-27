/// 最大公約数（greatest common divisor）を求める
///
/// ユークリッドの互除法
fn gcd(m: u64, n: u64) -> u64 {
    if n > m {
        gcd(n, m)
    } else if n == 0 {
        m
    } else {
        gcd(n, m % n)
    }
}

/// 最小公倍数（least common multiple）を求める
///
/// m * n == gcd(m, n) * lcm(m, n)から、
/// lcm == m * n / gcd(m, n)
/// オーバーフロー対策として、除算を先に行う
fn lcm(m: u64, n: u64) -> u64 {
    m / gcd(m, n) * n
}

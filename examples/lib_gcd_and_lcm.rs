/// 最大公約数（greatest common divisor）を求める
///
/// ユークリッドの互除法
fn gcd(mut x: usize, mut y: usize) -> usize {
    while y > 0 {
        let r = x % y;
        x = y;
        y = r;
    }
    return x;
}

/// 最小公倍数（least common multiple）を求める
///
/// m * n == gcd(m, n) * lcm(m, n)から、
/// lcm == m * n / gcd(m, n)
/// オーバーフロー対策として、除算を先に行う
fn lcm(m: usize, n: usize) -> usize {
    m / gcd(m, n) * n
}

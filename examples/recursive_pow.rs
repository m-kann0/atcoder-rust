/*
繰り返し2乗法
n^x (mod P)を高速に計算する
*/
fn main() {
    println!("pow(2, 8) = {}", pow(2, 8));
    println!("pow(2, 9) = {}", pow(2, 9));
    println!("pow(2, 10) = {}", pow(2, 10));
    println!("pow(2, 1_000_0000_000) = {}", pow(2, 1_000_000_000));
}

const MOD: i64 = 1_000_000_007;

fn pow(n: i64, x: i64) -> i64 {
    if x == 0 {
        return 1;
    }

    return if x % 2 == 0 {
        let t = pow(n, x / 2);
        t * t % MOD
    } else {
        n * pow(n, x - 1)
    }
}

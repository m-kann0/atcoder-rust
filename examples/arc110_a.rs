#![allow(non_snake_case)]

use std::io::Read;

fn main() {
    let mut buf = String::new();
    std::io::stdin().read_to_string(&mut buf).unwrap();

    let answer = solve(&buf);

    println!("{}", answer);
}

fn solve(input: &str) -> String {
    let mut iterator = input.split_whitespace();

    let n: usize = iterator.next().unwrap().parse().unwrap();

    let mut l = 2;
    for i in 3..=n {
        l = lcm(l, i);
    }

    let ans = l + 1;
    ans.to_string()
}

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

#[test]
fn test() {
    let cases: Vec<(&str, &str)> = vec![
        (
            r"3",
            "7"
        ),
        (
            r"10",
            "39916801"
        ),
    ];

    let mut all_ok = true;
    for (i, case) in cases.iter().enumerate() {
        print!("case {} : ", i);

        let expected = case.1;
        let actual = solve(case.0);

        if expected.trim() == actual.trim() {
            println!("OK");
        } else {
            println!("NG");
            println!("    Expected: {}", expected);
            println!("    Actual  : {}", actual);

            all_ok = false;
        }
    }

    assert_eq!(all_ok, true);
}
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
    let a: Vec<usize> = (0..n).map(|_| iterator.next().unwrap().parse().unwrap()).collect();

    let mut l = a[0];
    for i in 1..n {
        l = lcm(l, a[i]);
    }
    let m = l - 1;
    let mut ans = 0;
    for i in 0..n {
        ans += m % a[i];
    }
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
    let l = (m / gcd(m, n)).overflowing_mul(n);
    if l.1 {
        panic!();
    }
    l.0
}


#[test]
fn test() {
    let cases: Vec<(&str, &str)> = vec![
        (
            r"3
3 4 6",
            "10"
        ),
        (
            r"5
7 46 11 20 11",
            "90"
        ),
        (
            r"7
994 518 941 851 647 2 581",
            "4527"
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
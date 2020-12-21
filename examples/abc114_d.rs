#![allow(non_snake_case)]

use std::io::Read;
use std::collections::HashMap;

fn main() {
    let mut buf = String::new();
    std::io::stdin().read_to_string(&mut buf).unwrap();

    let answer = solve(&buf);

    println!("{}", answer);
}

fn solve(input: &str) -> String {
    let mut iterator = input.split_whitespace();

    // let mut factors = prime_factorization(32400);
    // eprintln!("factors of 32400 = {:?}", factors);
    // let mut factors = prime_factorization(3628800);
    // eprintln!("factors of 3628800 = {:?}", factors);

    let n: usize = iterator.next().unwrap().parse().unwrap();
    let mut factors = HashMap::new();
    for i in 1..=n {
        let f = prime_factorization(i);
        for (&k, &v) in f.iter() {
            *factors.entry(k).or_insert(0) += v;
        }
    }

    let mut a = Vec::new();
    for (&_k, &v) in factors.iter() {
        a.push(v);
    }

    let mut ans: usize = 0;

    for i in 0..a.len() {
        if a[i] < 2 {
            continue;
        }
        let mut count = 0;
        for j in 0..a.len() {
            if i == j {
                continue;
            }
            if a[j] >= 4 {
                count += 1;
            }
        }
        if count >= 2 {
            ans += comb(count, 2);
        }
    }

    for i in 0..a.len() {
        if a[i] < 14 {
            continue;
        }
        let mut count = 0;
        for j in 0..a.len() {
            if i == j {
                continue;
            }
            if a[j] >= 4 {
                count += 1;
            }
        }
        ans += count;
    }

    for i in 0..a.len() {
        if a[i] < 24 {
            continue;
        }
        let mut count = 0;
        for j in 0..a.len() {
            if i == j {
                continue;
            }
            if a[j] >= 2 {
                count += 1;
            }
        }
        ans += count;
    }

    for i in 0..a.len() {
        if a[i] >= 74 {
            ans += 1;
        }
    }

    ans.to_string()
}

/// 素因数分解
fn prime_factorization(mut n: usize) -> std::collections::HashMap<usize, usize> {
    let mut result: std::collections::HashMap<_, _> = std::collections::HashMap::new();

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

fn comb(n: usize, r: usize) -> usize {
    let mut n = n;
    let mut r = if r > n / 2 { n - r } else { r };

    let mut numerator = 1;
    let mut denominator = 1;
    while r > 0 {
        numerator *= n;
        denominator *= r;

        n -= 1;
        r -= 1;
    }
    numerator / denominator
}


#[test]
fn test() {
    let cases: Vec<(&str, &str)> = vec![
        (
            r"9",
            "0"
        ),
        (
            r"10",
            "1"
        ),
        (
            r"100",
            "543"
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
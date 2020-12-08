#![allow(non_snake_case)]

use std::io::Read;
use std::cmp::{min, max};
use rand::Rng;

fn main() {
    let mut buf = String::new();
    std::io::stdin().read_to_string(&mut buf).unwrap();

    let answer = solve(&buf);

    println!("{}", answer);
}

fn solve(input: &str) -> String {
    let mut iterator = input.split_whitespace();

    let n: usize = iterator.next().unwrap().parse().unwrap();
    let k: usize = iterator.next().unwrap().parse().unwrap();
    let a: Vec<usize> = (0..n).map(|_| iterator.next().unwrap().parse().unwrap()).collect();

    let mut ans = std::usize::MAX;
    for bit in 0..(1<<n) {
        let count = (bit as usize).count_ones();
        if count as usize != k {
            continue;
        }
        let mut now = 0;
        let mut a = a.clone();
        for i in 0..n {
            if (bit >> i) & 1 > 0 {
                // eprintln!("a[i] = {:?}", a[i]);
                let mut tall = 0;
                for j in 0..i {
                    tall = max(tall, a[j]);
                }
                // eprintln!("tall = {:?}", tall);
                if a[i] <= tall {
                    now += tall - a[i] + 1;
                    a[i] = tall + 1;
                }
            }
        }
        // eprintln!("bit = {:05b}", bit);
        // eprintln!("now = {:?}", now);
        ans = min(ans, now);
    }
    ans.to_string()
}

fn solve2(input: &str) -> String {
    let mut iterator = input.split_whitespace();

    let n: usize = iterator.next().unwrap().parse().unwrap();
    let k: usize = iterator.next().unwrap().parse().unwrap();
    let a: Vec<usize> = (0..n).map(|_| iterator.next().unwrap().parse().unwrap()).collect();

    let mut ans = std::usize::MAX;
    for bit in 0..(1<<n) {
        let count = (bit as usize).count_ones();
        if count as usize != k {
            continue;
        }

        // eprintln!("bit = {:05b}", bit);
        let mut now = 0;
        let mut prev = 0;
        for i in 0..n {
            if bit>>i & 1 > 0 {
                // eprintln!("prev = {:?}", prev);
                // eprintln!("a[i] = {:?}", a[i]);
                if a[i] <= prev {
                    now += prev - a[i] + 1;
                    prev += 1;
                } else {
                    prev = a[i];
                }
            } else {
                prev = max(prev, a[i]);
            }
            // eprintln!("now = {:?}", now);
        }
        ans = min(ans, now);
        // eprintln!("ans = {:?}", ans);
    }
    ans.to_string()
}

#[test]
fn random_test() {
    let mut rng = rand::thread_rng();
    for _ in 0..10 {
        let n = rng.gen_range(1, 16) as usize;
        let k = rng.gen_range(1, n + 1) as usize;
        let mut input = String::new();
        input.push_str(&format!("{} {}\n", n, k));
        for _ in 0..n {
            let a = rng.gen_range(1, 2) as usize;
            input.push_str(&format!("{} ", a));
        }
        let ans1 = solve(&input);
        let ans2 = solve2(&input);
        if ans1 != ans2 {
            eprintln!("input = {:?}", input);
            eprintln!("ans1 = {:?}", ans1);
            eprintln!("ans2 = {:?}", ans2);
        }
    }
}

#[test]
fn test() {
    let cases: Vec<(&str, &str)> = vec![
        (
            r"5 5
3949 3774 3598 3469 3424",
            "1541"
        ),
        (
            r"5 3
7 4 2 6 4",
            "7"
        ),
        (
            r"1 1
7",
            "0"
        ),
        (
            r"2 1
7 5",
            "0"
        ),
        (
            r"2 1
7 10",
            "0"
        ),
        (
            r"5 3
7 3 2 6 4",
            "7"
        ),
        (
            r"5 3
7 4 2 6 3",
            "7"
        ),
        (
            r"3 3
1 2 3",
            "0"
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
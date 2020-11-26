#![allow(non_snake_case)]

use std::io::Read;
use std::collections::BinaryHeap;
use std::cmp::{Reverse, max};
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
    let a: Vec<isize> = (0..3 * n).map(|_| iterator.next().unwrap().parse().unwrap()).collect();

    let mut f = vec![0; n + 1];
    {
        let mut pq = BinaryHeap::new();
        let mut sum = 0;
        for i in 0..n {
            pq.push(Reverse(a[i]));
            sum += a[i];
        }
        f[0] = sum;
        for i in 0..n {
            pq.push(Reverse(a[n + i]));
            sum += a[n + i];
            sum -= pq.pop().unwrap().0;
            f[i + 1] = sum;
        }
    }

    let mut b = vec![0; n + 1];
    {
        let mut pq = BinaryHeap::new();
        let mut sum = 0;
        for i in 0..n {
            pq.push(a[2 * n + i]);
            sum += a[2 * n + i];
        }
        b[n] = sum;
        for i in 0..n {
            pq.push(a[2 * n - 1 - i]);
            sum += a[2 * n - 1 - i];
            sum -= pq.pop().unwrap();
            b[n - 1 - i] = sum;
        }
    }

    let mut ans = std::isize::MIN;
    for i in 0..=n {
        ans = max(ans, f[i] - b[i]);
    }
    ans.to_string()
}

fn solve2(input: &str) -> String {
    let mut iterator = input.split_whitespace();

    let n: usize = iterator.next().unwrap().parse().unwrap();
    let a: Vec<isize> = (0..3 * n).map(|_| iterator.next().unwrap().parse().unwrap()).collect();
    let mut ans = std::isize::MIN;
    for i in 0..(1<<3 * n) {
        let mut count = 0;
        for j in 0..3 * n {
            if (i >> j) & 1 > 0 {
                count += 1;
            }
        }
        if count != 2 * n {
            continue;
        }
        let mut f_count = 0;
        let mut f = 0;
        let mut b = 0;
        for j in 0..3 * n {
            if (i >> j) & 1 > 0 {
                if f_count < n {
                    f_count += 1;
                    f += a[j];
                } else {
                    b += a[j];
                }
            }
        }
        // println!("{:09b}: {} - {} = {}", i, f, b, f - b);
        ans = max(ans, f - b);
    }
    ans.to_string()
}

#[test]
fn test3() {
    let input = "3
7 38 43 18 10 80 73 75 63";

    let ans1 = solve(input);
    let ans2 = solve2(input);
    eprintln!("ans1 = {:?}", ans1);
    eprintln!("ans2 = {:?}", ans2);
}

#[test]
fn test2() {
    let mut rng = rand::thread_rng();
    for _ in 0..10 {
        let n = 5;
        let mut input = String::new();
        input.push_str(&format!("{}\n", n));
        for _ in 0..3 * n {
            let ai: usize = rng.gen_range(1, 100);
            input.push_str(&format!("{} ", ai));
        }
        let input = input.trim();
        let ans1 = solve(input);
        let ans2 = solve2(input);
        if ans1 != ans2 {
            println!("input: {}", input);
            println!("ans1: {}", ans1);
            println!("ans2: {}", ans2);
        }
    }
}

#[test]
fn test() {
    let cases: Vec<(&str, &str)> = vec![
        (
            r"2
3 1 4 1 5 9",
            "1"
        ),
        (
            r"1
1 2 3",
            "-1"
        ),
        (
            r"3
8 2 2 7 4 6 5 3 8",
            "5"
        ),
        (
            r"2
9 6 5 4 9 4",
            "7"
        ),
    ];

    let mut all_ok = true;
    for (i, case) in cases.iter().enumerate() {
        print!("case {} : ", i);

        let expected = case.1;
        let actual = solve(case.0);

        if expected == actual {
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
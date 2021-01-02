#![allow(non_snake_case)]

use std::io::Read;
use std::cmp::min;

fn main() {
    let mut buf = String::new();
    std::io::stdin().read_to_string(&mut buf).unwrap();

    let answer = solve(&buf);

    println!("{}", answer);
}

fn solve(input: &str) -> String {
    let mut iterator = input.split_whitespace();

    let n: usize = iterator.next().unwrap().parse().unwrap();
    let _t: usize = iterator.next().unwrap().parse().unwrap();
    let a: Vec<usize> = (0..n).map(|_| iterator.next().unwrap().parse().unwrap()).collect();

    let mut diff = 0;
    let mut count = 1;
    let mut mn = a[0];
    for i in 1..n {
        mn = min(mn, a[i]);
        let diff_now = a[i] - mn;
        if diff_now > diff {
            diff = diff_now;
            count = 1;
        } else if diff_now == diff {
            count += 1;
        }
    }
    count.to_string()
}

#[test]
fn test() {
    let cases: Vec<(&str, &str)> = vec![
        (
            r"3 2
100 50 200",
            "1"
        ),
        (
            r"5 8
50 30 40 10 20",
            "2"
        ),
        (
            r"10 100
7 10 4 5 9 3 6 8 2 1",
            "2"
        ),
        (
            r"2 2
50 51",
            "1"
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
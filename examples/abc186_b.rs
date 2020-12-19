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

    let h: usize = iterator.next().unwrap().parse().unwrap();
    let w: usize = iterator.next().unwrap().parse().unwrap();
    let a: Vec<usize> = (0..h*w).map(|_| iterator.next().unwrap().parse().unwrap()).collect();

    let mut m = 101;
    for i in 0..h * w {
        m = min(m, a[i]);
    }

    let mut ans: usize = 0;
    for i in 0..h * w {
        if a[i] > m {
            ans += a[i] - m;
        }
    }
    ans.to_string()
}

#[test]
fn test() {
    let cases: Vec<(&str, &str)> = vec![
        (
            r"2 3
2 2 3
3 2 2",
            "2"
        ),
        (
            r"3 3
99 99 99
99 0 99
99 99 99",
            "792"
        ),
        (
            r"3 2
4 4
4 4
4 4",
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
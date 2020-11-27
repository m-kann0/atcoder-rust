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
    let k: usize = iterator.next().unwrap().parse().unwrap();
    let a: Vec<usize> = (0..n).map(|_| iterator.next().unwrap().parse().unwrap()).collect();

    let mut dp = vec![false; k + 1];
    for i in 1..=k {
        for j in 0..n {
            if i >= a[j] {
                dp[i] |= !dp[i - a[j]];
            }
        }
    }
    if dp[k] {
        "First".to_string()
    } else {
        "Second".to_string()
    }
}

#[test]
fn test() {
    let cases: Vec<(&str, &str)> = vec![
        (
            r"2 4
2 3",
            "First"
        ),
        (
            r"2 5
2 3",
            "Second"
        ),
        (
            r"2 7
2 3",
            "First"
        ),
        (
            r"3 20
1 2 3",
            "Second"
        ),
        (
            r"3 21
1 2 3",
            "First"
        ),
        (
            r"1 100000
1",
            "Second"
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
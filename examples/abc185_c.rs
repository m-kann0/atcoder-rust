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

    let l: u128 = iterator.next().unwrap().parse().unwrap();

    let ans = comb(l - 1, 11);
    ans.to_string()
}

fn comb(n: u128, r: u128) -> u128 {
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
            r"12",
            "1"
        ),
        (
            r"13",
            "12"
        ),
        (
            r"17",
            "4368"
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
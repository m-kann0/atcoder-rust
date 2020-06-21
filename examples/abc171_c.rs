use std::io::Read;
use itertools::Itertools;

fn main() {
    let mut buf = String::new();
    std::io::stdin().read_to_string(&mut buf).unwrap();

    let answer = solve(&buf);

    println!("{}", answer);
}

fn solve(input: &str) -> String {
    let mut iterator = input.split_whitespace();

    let mut n: usize = iterator.next().unwrap().parse().unwrap();

    let mut result = String::new();
    while n > 0 {
        n -= 1;
        let amari = n % 26;
        let c = ('a' as u8 + amari as u8) as char;
        result.push(c);
        n /= 26;
    }

    let result = result.chars().rev().map(|c| c.to_string()).join("");
    return result;
}

#[test]
fn test() {
    let cases: Vec<(&str, &str)> = vec![
        (
            r"2",
            "b"
        ),
        (
            r"26",
            "z"
        ),
        (
            r"27",
            "aa"
        ),
        (
            r"28",
            "ab"
        ),
        (
            r"123456789",
            "jjddja"
        ),
        (
            r"701",
            "zy"
        ),
        (
            r"702",
            "zz"
        ),
        (
            r"18277",
            "zzy"
        ),
        (
            r"18278",
            "zzz"
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
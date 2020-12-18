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

    let d: usize = iterator.next().unwrap().parse().unwrap();
    let n: usize = iterator.next().unwrap().parse().unwrap();

    let mut count = 0;
    for i in 1usize.. {
        let mut is_ok = true;
        let mut x = i;
        for _ in 0..d {
            if x % 100 != 0 {
                is_ok = false;
                break;
            }
            x /= 100;
        }
        if is_ok && x % 100 != 0 {
            count += 1;
        }
        if count == n {
            return i.to_string();
        }
    }

    unreachable!();
}

#[test]
fn test() {
    let cases: Vec<(&str, &str)> = vec![
        (
            r"0 5",
            "5"
        ),
        (
            r"1 11",
            "1100"
        ),
        (
            r"2 85",
            "850000"
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
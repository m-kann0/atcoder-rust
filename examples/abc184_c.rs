#![allow(non_snake_case)]

use std::io::Read;
use std::mem::swap;

fn main() {
    let mut buf = String::new();
    std::io::stdin().read_to_string(&mut buf).unwrap();

    let answer = solve(&buf);

    println!("{}", answer);
}

fn solve(input: &str) -> String {
    let mut iterator = input.split_whitespace();

    let mut a: isize = iterator.next().unwrap().parse().unwrap();
    let mut b: isize = iterator.next().unwrap().parse().unwrap();
    let mut c: isize = iterator.next().unwrap().parse().unwrap();
    let mut d: isize = iterator.next().unwrap().parse().unwrap();

    if a == c && b == d {
        return "0".to_string();
    }

    if a + b == c + d || a - b == c - d || (a - c).abs() + (b - d).abs() <= 3 {
        return "1".to_string();
    }

    if a == c {
        return if (b - d).abs() % 2 == 0 {
            "2".to_string()
        } else {
            "3".to_string()
        }
    }
    if b == d {
        return if (a - c).abs() % 2 == 0 {
            "2".to_string()
        } else {
            "3".to_string()
        }
    }

    if a > c {
        swap(&mut a, &mut c);
        swap(&mut b, &mut d);
    }


    if b > d {
        a += b - d;
        b -= b - d;
    } else {
        a += d - b;
        b += d - b;
    }

    if a + b == c + d || a - b == c - d || (a - c).abs() + (b - d).abs() <= 3 {
        return "2".to_string();
    }

    if (a - c).abs() % 2 == 0 {
        "2".to_string()
    } else {
        "3".to_string()
    }
}

#[test]
fn test() {
    let cases: Vec<(&str, &str)> = vec![
        (
            r"1 1
5 6",
            "2"
        ),
        (
            r"1 1
1 200001",
            "2"
        ),
        (
            r"2 3
998244353 998244853",
            "3"
        ),
        (
            r"1 1
1 1",
            "0"
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
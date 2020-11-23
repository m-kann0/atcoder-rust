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

    let x1: isize = iterator.next().unwrap().parse().unwrap();
    let y1: isize = iterator.next().unwrap().parse().unwrap();
    let x2: isize = iterator.next().unwrap().parse().unwrap();
    let y2: isize = iterator.next().unwrap().parse().unwrap();

    let x = (x2 - x1).abs();
    let y = (y2 - y1).abs();

    if x == 0 && y == 0 {
        return "0".to_string();
    }
    if x == y {
        return "1".to_string();
    }
    if x + y <= 3 {
        return "1".to_string();
    }
    if (x + y) % 2 == 0 {
        return "2".to_string();
    }
    if (x - y).abs() <= 3 {
        return "2".to_string();
    }
    if x + y <= 6 {
        return "2".to_string();
    }
    return "3".to_string();
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
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

    let a: isize = iterator.next().unwrap().parse().unwrap();
    let b: isize = iterator.next().unwrap().parse().unwrap();
    let k: isize = iterator.next().unwrap().parse().unwrap();

    if a + k - 1 >= b - k + 1 {
        let mut result = String::new();
        for i in a..=b {
            result.push_str(&format!("{}\n", i));
        }
        return result;
    }

    let mut result = String::new();
    for i in a..=a + k - 1 {
        result.push_str(&format!("{}\n", i));
    }
    for i in b - k + 1..=b {
        result.push_str(&format!("{}\n", i));
    }
    result
}

#[test]
fn test() {
    let cases: Vec<(&str, &str)> = vec![
        (
            r"3 8 2",
            "3
4
7
8"
        ),
        (
            r"4 8 3",
            "4
5
6
7
8"
        ),
        (
            r"2 9 100",
            "2
3
4
5
6
7
8
9"
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
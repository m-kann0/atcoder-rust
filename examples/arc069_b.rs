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
    let s: Vec<char> = iterator.next().unwrap().chars().collect();

    for first in 0..2 {
        for second in 0..2 {
            let mut sheep = vec![false; n];
            sheep[0] = first == 0;
            sheep[1] = second == 0;
            for i in 2..n {
                if sheep[i - 1] {
                    if s[i - 1] == 'o' {
                        sheep[i] = sheep[i - 2];
                    } else {
                        sheep[i] = !sheep[i - 2];
                    }
                } else {
                    if s[i - 1] == 'o' {
                        sheep[i] = !sheep[i - 2];
                    } else {
                        sheep[i] = sheep[i - 2];
                    }
                }
            }

            let mut is_ok = true;
            for i in 0..n {
                let left = (i + n - 1) % n;
                let right = (i + 1) % n;
                if sheep[i] {
                    if s[i] == 'o' {
                        if sheep[left] != sheep[right] {
                            is_ok = false;
                        }
                    } else {
                        if sheep[left] == sheep[right] {
                            is_ok = false;
                        }
                    }
                } else {
                    if s[i] == 'o' {
                        if sheep[left] == sheep[right] {
                            is_ok = false;
                        }
                    } else {
                        if sheep[left] != sheep[right] {
                            is_ok = false;
                        }
                    }
                }
            }
            if is_ok {
                let mut result = String::new();
                for i in 0..n {
                    let c = if sheep[i] { 'S' } else { 'W' };
                    result.push(c);
                }
                return result;
            }
        }
    }

    "-1".to_string()
}

#[test]
fn test() {
    let cases: Vec<(&str, &str)> = vec![
        (
            r"6
ooxoox",
            "SSSWWS"
        ),
        (
            r"3
oox",
            "-1"
        ),
        (
            r"10
oxooxoxoox",
            "SSWWSSSWWS"
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
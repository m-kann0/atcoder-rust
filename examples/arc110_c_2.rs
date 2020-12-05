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
    let mut p: Vec<usize> = (0..n).map(|_| iterator.next().unwrap().parse().unwrap()).collect();

    let mut result = String::new();
    let mut begin = 0;
    let mut end = 0;
    while begin < n - 1 {
        while end < n {
            if p[end] == begin + 1 {
                break;
            }
            end += 1;
        }
        if begin == end {
            return "-1".to_string();
        }
        for i in (begin..end).rev() {
            p.swap(i, i + 1);
            result.push_str(&format!("{}\n", i + 1));
        }
        for i in begin..end {
            if p[i] != i + 1 {
                return "-1".to_string();
            }
        }
        begin = end;
    }
    result
}

#[test]
fn test() {
    let cases: Vec<(&str, &str)> = vec![
        (
            r"5
2 4 1 5 3",
            "4
2
3
1"
        ),
        (
            r"5
5 4 3 2 1",
            "-1"
        ),
        (
            r"5
1 2 3 4 5",
            "-1"
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
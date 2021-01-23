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
    let x: usize = iterator.next().unwrap().parse().unwrap();
    let mut v: Vec<usize> = Vec::new();
    let mut p: Vec<usize> = Vec::new();
    for _ in 0..n {
        let vi: usize = iterator.next().unwrap().parse().unwrap();
        let pi: usize = iterator.next().unwrap().parse().unwrap();
        v.push(vi);
        p.push(pi);
    }

    let x = x * 100;

    let mut current: usize = 0;
    for i in 0..n {
        current += v[i] * p[i];
        if current > x {
            return (i + 1).to_string();
        }
    }
    return "-1".to_string()
}

#[test]
fn test() {
    let cases: Vec<(&str, &str)> = vec![
        (
            r"2 15
200 5
350 3",
            "2"
        ),
        (
            r"2 10
200 5
350 3",
            "2"
        ),
        (
            r"3 1000000
1000 100
1000 100
1000 100",
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
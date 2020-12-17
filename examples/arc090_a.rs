#![allow(non_snake_case)]

use std::io::Read;
use std::cmp::max;

fn main() {
    let mut buf = String::new();
    std::io::stdin().read_to_string(&mut buf).unwrap();

    let answer = solve(&buf);

    println!("{}", answer);
}

fn solve(input: &str) -> String {
    let mut iterator = input.split_whitespace();

    let n: usize = iterator.next().unwrap().parse().unwrap();

    let mut a = Vec::new();
    for _ in 0..2 {
        let line: Vec<usize> = (0..n).map(|_| iterator.next().unwrap().parse().unwrap()).collect();
        a.push(line);
    }

    let mut ans: usize = 0;
    for i in 0..n {
        let mut now: usize = 0;
        for j in 0..n {
            if j <= i {
                now += a[0][j];
            }
            if j >= i {
                now += a[1][j];
            }
        }
        ans = max(ans, now);
    }
    ans.to_string()
}

#[test]
fn test() {
    let cases: Vec<(&str, &str)> = vec![
        (
            r"5
3 2 2 4 1
1 2 2 2 1",
            "14"
        ),
        (
            r"4
1 1 1 1
1 1 1 1",
            "5"
        ),
        (
            r"7
3 3 4 5 4 5 3
5 3 4 4 2 3 2",
            "29"
        ),
        (
            r"1
2
3",
            "5"
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
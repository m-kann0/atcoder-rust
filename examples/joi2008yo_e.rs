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

    let r: usize = iterator.next().unwrap().parse().unwrap();
    let c: usize = iterator.next().unwrap().parse().unwrap();
    let mut a = vec![vec![0; c]; r];
    for i in 0..r {
        for j in 0..c {
            let aij: usize = iterator.next().unwrap().parse().unwrap();
            a[i][j] = aij;
        }
    }

    let mut ans: usize = 0;
    for bit in 0..(1 << r) {
        let mut now = 0;
        for i in 0..c {
            let mut count = 0;
            for j in 0..r {
                if bit>>j & 1 > 0 && a[j][i] == 0 ||
                    bit>>j & 1 == 0 && a[j][i] == 1 {
                    count += 1;
                }
            }
            now += max(count, r - count);
        }
        ans = max(ans, now);
    }
    ans.to_string()
}

#[test]
fn test() {
    let cases: Vec<(&str, &str)> = vec![
        (
            r"2 5
0 1 0 1 0
1 0 0 0 1",
            "9"
        ),
        (
            r"3 6
1 0 0 0 1 0
1 1 1 0 1 0
1 0 1 1 0 1",
            "15"
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
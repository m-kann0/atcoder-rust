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
    let a: Vec<usize> = (0..n).map(|_| iterator.next().unwrap().parse().unwrap()).collect();

    let mut count = vec![0; 100005];
    for i in 0..n {
        if a[i] != 0 {
            count[a[i] - 1] += 1;
        }
        count[a[i]] += 1;
        count[a[i] + 1] += 1;
    }

    let mut ans: usize = 0;
    for x in 0..=100000 {
        ans = max(ans, count[x]);
    }
    ans.to_string()
}

#[test]
fn test() {
    let cases: Vec<(&str, &str)> = vec![
        (
            r"7
3 1 4 1 5 9 2",
            "4"
        ),
        (
            r"10
0 1 2 3 4 5 6 7 8 9",
            "3"
        ),
        (
            r"1
99999",
            "1"
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
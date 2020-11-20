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
    let a: Vec<usize> = (0..n).map(|_| iterator.next().unwrap().parse().unwrap()).collect();

    const INF: usize = std::usize::MAX;
    let mut dp = vec![INF; n];
    for i in 0..n {
        let mut ok: isize = -1;
        let mut ng: isize = n as isize - 1;
        while (ok - ng).abs() > 1 {
            let mid = (ok + ng) / 2;
            if dp[mid as usize] < a[i] {
                ok = mid;
            } else {
                ng = mid;
            }
        }
        dp[(ok + 1) as usize] = a[i];
    }
    let ans = dp.iter().enumerate().filter(|(_i, x)| **x < INF).last().unwrap().0 + 1;
    ans.to_string()
}

#[test]
fn test() {
    let cases: Vec<(&str, &str)> = vec![
        (
            r"5
3 1 5 4 2",
            "2"
        ),
        (
            r"6
1 2 3 4 5 6",
            "6"
        ),
        (
            r"7
7 6 5 4 3 2 1",
            "1"
        ),
        (
            r"20
19 11 10 7 8 9 17 18 20 4 3 15 16 1 5 14 6 2 13 12",
            "6"
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

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
    let c: Vec<usize> = (0..n).map(|_| iterator.next().unwrap().parse().unwrap()).collect();

    const MOD: usize = 1_000_000_007;
    let mut dp = vec![0_usize; n];
    let mut last = vec![None; 200_005];
    dp[0] = 1;
    last[c[0]] = Some(0_usize);
    for i in 1..n {
        if let Some(j) = last[c[i]] {
            if j == i - 1 {
                dp[i] = dp[i - 1];
            } else {
                dp[i] = dp[i - 1] + dp[j];
            }
        } else {
            dp[i] = dp[i - 1];
        }
        dp[i] %= MOD;
        last[c[i]] = Some(i);
    }
    dp[n - 1].to_string()
}

#[test]
fn test() {
    let cases: Vec<(&str, &str)> = vec![
        (
            r"5
1
2
1
2
2",
            "3"
        ),
        (
            r"6
4
2
5
4
2
4",
            "5"
        ),
        (
            r"7
1
3
1
2
3
3
2",
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
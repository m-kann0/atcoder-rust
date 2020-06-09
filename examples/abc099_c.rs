use std::io::Read;
use std::cmp::min;

fn main() {
    let mut buf = String::new();
    std::io::stdin().read_to_string(&mut buf).unwrap();

    let answer = solve(&buf);

    println!("{}", answer);
}

fn solve(input: &str) -> String {
    let mut iterator = input.split_whitespace();

    let n: usize = iterator.next().unwrap().parse().unwrap();

    const INF: usize = 100_001;
    let mut dp: Vec<usize> = vec![INF; n + 1];
    dp[0] = 0;
    for i in 0..n {
        dp[i + 1] = min(dp[i + 1], dp[i] + 1);
        let mut x: usize = 6;
        while i + x <= n {
            dp[i + x] = min(dp[i + x], dp[i] + 1);
            x *= 6;
        }
        let mut y: usize = 9;
        while i + y <= n {
            dp[i + y] = min(dp[i + y], dp[i] + 1);
            y *= 9;
        }
    }
    return dp[n].to_string();
}

#[test]
fn test() {
    let cases: Vec<(&str, &str)> = vec![
        (
            r"127",
            "4"
        ),
        (
            r"3",
            "3"
        ),
        (
            r"44852",
            "16"
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
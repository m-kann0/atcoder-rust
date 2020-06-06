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
    let l: usize = iterator.next().unwrap().parse().unwrap();
    let mut hurdle: Vec<bool> = vec![false; l + 10];
    for _ in 0..n {
        let x: usize = iterator.next().unwrap().parse().unwrap();
        hurdle[x] = true;
    }
    let t1: usize = iterator.next().unwrap().parse().unwrap();
    let t2: usize = iterator.next().unwrap().parse().unwrap();
    let t3: usize = iterator.next().unwrap().parse().unwrap();

    const INF: usize = 1_000_000_000;
    let mut dp: Vec<usize> = vec![INF; l + 10];
    dp[0] = 0;
    for i in 0..l {
        // 行動1
        let d1 = if hurdle[i + 1] {
            t1 + t3
        } else {
            t1
        };
        dp[i + 1] = min(dp[i + 1], dp[i] + d1);

        // 行動2
        let d2 = if hurdle[i + 2] {
            t1 + t2 + t3
        } else {
            t1 + t2
        };
        dp[i + 2] = min(dp[i + 2], dp[i] + d2);

        // 行動3
        let d3 = if hurdle[i + 4] {
            t1 + 3 * t2 + t3
        } else {
            t1 + 3 * t2
        };
        dp[i + 4] = min(dp[i + 4], dp[i] + d3);
    }

    // 空中ゴールを考慮
    dp[l] = min(dp[l], dp[l - 1] + (t1 + t2) / 2);
    dp[l] = min(dp[l], dp[l - 2] + t1 / 2 + t2 + t2 / 2);
    if l >= 3 {
        dp[l] = min(dp[l], dp[l - 3] + t1 / 2 + t2 * 2 + t2 / 2);
    }

    return format!("{}", dp[l]);
}

#[test]
fn test() {
    let cases: Vec<(&str, &str)> = vec![
        (
            r"2 5
1 4
2 2 20",
            "10"
        ),
        (
            r"4 5
1 2 3 4
2 20 100",
            "164"
        ),
        (
            r"10 19
1 3 4 5 7 8 10 13 15 17
2 1000 10",
            "138"
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
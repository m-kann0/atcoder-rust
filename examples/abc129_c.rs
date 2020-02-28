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
    let m: usize = iterator.next().unwrap().parse().unwrap();

    let mut broken: Vec<bool> = vec![false; n + 1];
    for _i in 0..m {
        let a: usize = iterator.next().unwrap().parse().unwrap();
        broken[a] = true;
    }

    let mut dp: Vec<usize> = vec![0; n + 1];
    dp[n] = 1;
    dp[n - 1] = if broken[n - 1] { 0 } else { 1 };
    for i in (0..(n-1)).rev() {
        if broken[i] {
            dp[i] = 0;
            continue;
        }

        dp[i] = (dp[i + 1] + dp[i + 2]) % MOD;
    }

    return dp[0].to_string();
}

const MOD: usize = 1_000_000_007;

#[test]
fn test() {
    let cases: Vec<(&str, &str)> = vec![
        (
            r"6 1
3",
            "4"
        ),
        (
            r"10 2
4
5",
            "0"
        ),
        (
            r"100 5
1
23
45
67
89",
            "608200469"
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
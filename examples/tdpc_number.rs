use std::io::Read;

fn main() {
    let mut buf = String::new();
    std::io::stdin().read_to_string(&mut buf).unwrap();

    let answer = solve(&buf);

    println!("{}", answer);
}

fn solve(input: &str) -> String {
    let mut iterator = input.split_whitespace();

    let d: usize = iterator.next().unwrap().parse().unwrap();
    let n: Vec<usize> = iterator.next().unwrap().chars().map(|c| c as usize - '0' as usize).collect();

    const MOD: usize = 1_000_000_007;
    // dp[確定した桁数][N未満が確定しているかどうか][各桁の和 mod D]
    let mut dp: Vec<Vec<Vec<usize>>> = vec![vec![vec![0; d]; 2]; n.len() + 1];
    dp[0][0][0] = 1;
    for i in 0..n.len() {
        for j in 0..2 {
            for k in 0..d {
                let d_max = if j == 1 { 9 } else { n[i] };
                for digit in 0..=d_max {
                    dp[i + 1][(j == 1 || digit < d_max) as usize][(k + digit) % d] += dp[i][j][k];
                    dp[i + 1][(j == 1 || digit < d_max) as usize][(k + digit) % d] %= MOD;
                }
            }
        }
    }
    // 0の分も数えているので-1する
    let ans = (dp[n.len()][0][0] + dp[n.len()][1][0] + MOD - 1) % MOD;
    ans.to_string()
}

#[test]
fn test() {
    let cases: Vec<(&str, &str)> = vec![
        (
            r"3
100",
            "33"
        ),
        (
            r"7
123456789012345678901234567890",
            "468357804"
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
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
    let mut d: usize = iterator.next().unwrap().parse().unwrap();

    let mut dp: Vec<Vec<Vec<f64>>> = vec![vec![vec![0.0; 105]; 105]; 105];
    dp[0][0][0] = 1.0;

    for i in 0..n {
        let mut ndp: Vec<Vec<Vec<f64>>> = vec![vec![vec![0.0; 105]; 105]; 105];
        for a in 0..100 {
            for b in 0..100 {
                for c in 0..100 {
                    // 1
                    ndp[a][b][c] += dp[a][b][c] / 6.0;
                    // 2
                    ndp[a + 1][b][c] += dp[a][b][c] / 6.0;
                    // 3
                    ndp[a][b + 1][c] += dp[a][b][c] / 6.0;
                    // 4
                    ndp[a + 2][b][c] += dp[a][b][c] / 6.0;
                    // 5
                    ndp[a][b][c + 1] += dp[a][b][c] / 6.0;
                    // 6
                    ndp[a + 1][b + 1][c] += dp[a][b][c] / 6.0;
                }
            }
        }
        dp = ndp;
    }

    let mut d1 = 0;
    while d % 2 == 0 {
        d /= 2;
        d1 += 1;
    }
    let mut d2 = 0;
    while d % 3 == 0 {
        d /= 3;
        d2 += 1;
    }
    let mut d3 = 0;
    while d % 5 == 0 {
        d /= 5;
        d3 += 1;
    }

    if d != 1 {
        return format!("{}", 0.0);
    }

    let mut ans = 0.0;
    for a in 0..100 {
        if a < d1 { continue }
        for b in 0..100 {
            if b < d2 { continue }
            for c in 0..100 {
                if c < d3 { continue }
                ans += dp[a][b][c];
            }
        }
    }
    ans.to_string()
}

#[test]
fn test() {
    let cases: Vec<(&str, &str)> = vec![
        ("2 6", "0.416666667"),
        ("3 2", "0.875000000"),
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
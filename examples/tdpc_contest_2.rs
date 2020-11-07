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
    let p: Vec<usize> = (0..n).map(|_| iterator.next().unwrap().parse().unwrap()).collect();

    let mut dp: Vec<Vec<bool>> = vec![vec![false; 10000 + 1]; n + 1];
    dp[0][0] = true;

    // for i in 0..n {
    //     for j in 0..=10000 {
    //         dp[i + 1][j] = dp[i][j];
    //         if j >= p[i] {
    //             dp[i + 1][j] |= dp[i][j - p[i]];
    //         }
    //     }
    // }

    for i in 0..n {
        for j in 0..=10000 {
            if j < p[i] {
                dp[i + 1][j] = dp[i][j];
            } else {
                dp[i + 1][j] = dp[i][j] || dp[i][j - p[i]];
            }
        }
    }

    // for i in 1..=n {
    //     for j in 0..=10000 {
    //         if j < p[i - 1] {
    //             dp[i][j] = dp[i - 1][j];
    //         } else {
    //             dp[i][j] = dp[i - 1][j] || dp[i - 1][j - p[i - 1]];
    //         }
    //     }
    // }

    let ans = dp[n].iter().filter(|it| **it).count();
    ans.to_string()
}

#[test]
fn test() {
    let cases: Vec<(&str, &str)> = vec![
        (
            r"3
2 3 5",
            r"7",
        ),
        (
            r"10
1 1 1 1 1 1 1 1 1 1",
            r"11",
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
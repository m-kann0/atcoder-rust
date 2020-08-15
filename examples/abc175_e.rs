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
    let k: usize = iterator.next().unwrap().parse().unwrap();
    let mut map: Vec<Vec<usize>> = vec![vec![0; c]; r];
    for _ in 0..k {
        let ri: usize = iterator.next().unwrap().parse().unwrap();
        let ci: usize = iterator.next().unwrap().parse().unwrap();
        let vi: usize = iterator.next().unwrap().parse().unwrap();
        map[ri - 1][ci - 1] = vi;
    }

    let mut dp: Vec<Vec<Vec<usize>>> = vec![vec![vec![0; 4]; 3005]; 3005];
    for i in 0..r {
        for j in 0..c {
            for k in (0..3).rev() {
                dp[i][j][k + 1] = max(dp[i][j][k + 1], dp[i][j][k] + map[i][j]);
            }
            for k in 0..4 {
                dp[i + 1][j][0] = max(dp[i + 1][j][0], dp[i][j][k]);
                dp[i][j + 1][k] = max(dp[i][j + 1][k], dp[i][j][k]);
            }
        }
    }

    let mut ans = 0;
    for k in 0..4 {
        ans = max(ans, dp[r - 1][c - 1][k]);
    }
    ans.to_string()
}

#[test]
fn test() {
    let cases: Vec<(&str, &str)> = vec![
        (
            r"2 2 3
1 1 3
2 1 4
1 2 5",
            "8"
        ),
        (
            r"2 5 5
1 1 3
2 4 20
1 2 1
1 3 4
1 4 2",
            "29"
        ),
        (
            r"4 5 10
2 5 12
1 5 12
2 3 15
1 2 20
1 1 28
2 4 26
3 2 27
4 5 21
3 5 10
1 3 10",
            "142"
        ),
        (
            r"4 5 11
2 5 12
1 5 12
2 3 15
1 2 20
1 1 28
2 4 26
3 2 27
4 5 21
3 5 10
1 3 10
4 1 1000",
            "1049"
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
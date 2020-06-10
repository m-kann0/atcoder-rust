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
    let w_limit: usize = iterator.next().unwrap().parse().unwrap();
    let mut w: Vec<usize> = Vec::new();
    let mut v: Vec<usize> = Vec::new();
    for _ in 0..n {
        w.push(iterator.next().unwrap().parse().unwrap());
        v.push(iterator.next().unwrap().parse().unwrap());
    }

    let mut dp: Vec<Vec<usize>> = vec![vec![0; w_limit + 1]; n + 1];
    for i in 0..n {
        for j in 1..(w_limit + 1) {
            dp[i + 1][j] = dp[i][j];
            if j >= w[i] {
                dp[i + 1][j] = max(dp[i + 1][j], dp[i][j - w[i]] + v[i]);
            }
        }
    }
    return dp[n][w_limit].to_string();
}

#[test]
fn test() {
    let cases: Vec<(&str, &str)> = vec![
        (
            r"3 8
3 30
4 50
5 60",
            "90"
        ),
        (
            r"5 5
1 1000000000
1 1000000000
1 1000000000
1 1000000000
1 1000000000",
            "5000000000"
        ),
        (
            r"6 15
6 5
5 6
6 4
6 6
3 5
7 2",
            "17"
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
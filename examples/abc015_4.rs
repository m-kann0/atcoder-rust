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

    let w: usize = iterator.next().unwrap().parse().unwrap();
    let n: usize = iterator.next().unwrap().parse().unwrap();
    let m: usize = iterator.next().unwrap().parse().unwrap();
    let mut a: Vec<usize> = Vec::new();
    let mut b: Vec<usize> = Vec::new();
    for _ in 0..n {
        let ai: usize = iterator.next().unwrap().parse().unwrap();
        let bi: usize = iterator.next().unwrap().parse().unwrap();
        a.push(ai);
        b.push(bi);
    }

    let mut dp: Vec<Vec<Vec<usize>>> = vec![vec![vec![0; w + 1]; m + 1]; n + 1];
    for i in 0..n {
        for j in 0..=m {
            for k in 0..=w {
                if j < 1 || k < a[i] {
                    dp[i + 1][j][k] = dp[i][j][k];
                } else {
                    dp[i + 1][j][k] = max(
                        dp[i][j][k],
                        dp[i][j - 1][k - a[i]] + b[i]
                    );
                }
            }
        }
    }

    let mut ans: usize = 0;
    for j in 0..=m {
        for k in 0..=w {
            ans = max(ans, dp[n][j][k]);
        }
    }
    ans.to_string()
}

#[test]
fn test() {
    let cases: Vec<(&str, &str)> = vec![
        (
            r"10
3 2
4 20
3 40
6 100",
            "140"
        ),
        (
            r"10
5 4
9 10
3 7
3 1
2 6
4 5",
            "18"
        ),
        (
            r"22
5 3
5 40
8 50
3 60
4 70
6 80",
            "210"
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
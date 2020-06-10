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
    let mut a: Vec<usize> = Vec::new();
    let mut b: Vec<usize> = Vec::new();
    let mut c: Vec<usize> = Vec::new();
    for _ in 0..n {
        a.push(iterator.next().unwrap().parse().unwrap());
        b.push(iterator.next().unwrap().parse().unwrap());
        c.push(iterator.next().unwrap().parse().unwrap());
    }

    let mut dp: Vec<Vec<usize>> = vec![vec![0; 3]; n + 1];
    for i in 0..n {
        dp[i + 1][0] = max(dp[i][1] + a[i], dp[i][2] + a[i]);
        dp[i + 1][1] = max(dp[i][0] + b[i], dp[i][2] + b[i]);
        dp[i + 1][2] = max(dp[i][0] + c[i], dp[i][1] + c[i]);
    }
    let ans = max(dp[n][0], max(dp[n][1], dp[n][2]));
    return ans.to_string();
}

#[test]
fn test() {
    let cases: Vec<(&str, &str)> = vec![
        (
            r"3
10 40 70
20 50 80
30 60 90",
            "210"
        ),
        (
            r"1
100 10 1",
            "100"
        ),
        (
            r"7
6 7 8
8 8 3
2 5 2
7 8 6
4 6 8
2 3 4
7 5 1",
            "46"
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
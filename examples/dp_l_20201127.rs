#![allow(non_snake_case)]

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
    let a: Vec<isize> = (0..n).map(|_| iterator.next().unwrap().parse().unwrap()).collect();

    let mut dp = vec![vec![None; n + 1]; n + 1];
    for i in 0..=n {
        for j in 0..=i {
            dp[i][j] = Some((0, 0));
        }
    }

    let result = rec(n, &a, 0, n, &mut dp);
    (result.0 - result.1).to_string()
}

fn rec(n: usize, a: &Vec<isize>, l: usize, r: usize, dp: &mut Vec<Vec<Option<(isize, isize)>>>) -> (isize, isize) {
    if let Some((x, y)) = dp[l][r] {
        return (x, y);
    }

    let result1 = rec(n, a, l + 1, r, dp);
    let result1 = (result1.1 + a[l], result1.0);
    let result2 = rec(n, a, l, r - 1, dp);
    let result2 = (result2.1 + a[r - 1], result2.0);
    let result = if result1.0 - result1.1 > result2.0 - result2.1 {
        result1
    } else {
        result2
    };
    dp[l][r] = Some(result);
    result
}

#[test]
fn test() {
    let cases: Vec<(&str, &str)> = vec![
        (
            r"4
10 80 90 30",
            "10"
        ),
        (
            r"3
10 100 10",
            "-80"
        ),
        (
            r"1
10",
            "10"
        ),
        (
            r"10
1000000000 1 1000000000 1 1000000000 1 1000000000 1 1000000000 1",
            "4999999995"
        ),
        (
            r"6
4 2 9 7 1 5",
            "2"
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
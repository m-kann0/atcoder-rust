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
    let c: Vec<usize> = (0..n).map(|_| iterator.next().unwrap().parse().unwrap()).collect();

    let lis = lis::lis(&c, std::usize::MAX);
    let ans = n - lis;
    ans.to_string()
}

mod lis {
    pub fn lis<T: PartialOrd + PartialEq + Copy + Clone>(v: &Vec<T>, inf: T) -> usize {
        let mut dp = vec![inf; v.len()];
        for &x in v {
            let mut ok: isize = v.len() as isize - 1;
            let mut ng: isize = -1;
            while (ok - ng).abs() > 1 {
                let mid = (ok + ng) / 2;
                if x <= dp[mid as usize] {
                    ok = mid;
                } else {
                    ng = mid;
                }
            }
            dp[ok as usize] = x.clone();
        }

        let mut ans = 0;
        for i in 0..v.len() {
            if dp[i] < inf {
                ans = i + 1;
            }
        }
        ans
    }
}

#[test]
fn test() {
    let cases: Vec<(&str, &str)> = vec![
        (
            r"6
1
3
5
2
4
6",
            "2"
        ),
        (
            r"5
5
4
3
2
1",
            "4"
        ),
        (
            r"7
1
2
3
4
5
6
7",
            "0"
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

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
    let k: usize = iterator.next().unwrap().parse().unwrap();
    let a: Vec<usize> = (0..n).map(|_| iterator.next().unwrap().parse().unwrap()).collect();

    let mut sum = vec![0; n + 1];
    for i in 0..n {
        sum[i + 1] = sum[i] + a[i];
    }

    let mut candidates = Vec::new();
    for i in 0..n {
        for j in 0..n {
            if i > j {
                continue;
            }
            candidates.push(sum[j + 1] - sum[i]);
        }
    }

    let mut ans: usize = 0;
    for i in (0..40).rev() {
        let mut now = vec![];
        for &c in candidates.iter() {
            if (c & (1 << i)) > 0 {
                now.push(c);
            }
        }
        if now.len() >= k {
            ans += 1 << i;
            candidates = now;
        }
    }
    ans.to_string()
}

#[test]
fn test() {
    let cases: Vec<(&str, &str)> = vec![
        (
            r"4 2
2 5 2 5",
            "12"
        ),
        (
            r"8 4
9 1 8 2 7 5 6 4",
            "32"
        ),
    ];

    let mut all_ok = true;
    for (i, case) in cases.iter().enumerate() {
        print!("case {} : ", i);

        let expected = case.1;
        let actual = solve(case.0);

        if expected.trim() == actual.trim() {
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
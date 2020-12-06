#![allow(non_snake_case)]

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
    let m: usize = iterator.next().unwrap().parse().unwrap();
    let mut a = vec![vec![0; m]; n];
    for i in 0..n {
        for j in 0..m {
            let aij: usize = iterator.next().unwrap().parse().unwrap();
            a[i][j] = aij;
        }
    }

    let mut ans = 0;
    for t1 in 0..m {
        for t2 in 0..m {
            if t1 == t2 {
                continue;
            }
            let mut now = 0;
            for i in 0..n {
                now += max(a[i][t1], a[i][t2]);
            }
            ans = max(ans, now);
        }
    }
    ans.to_string()
}

#[test]
fn test() {
    let cases: Vec<(&str, &str)> = vec![
        (
            r"1 2
80 84",
            "84"
        ),
        (
            r"3 4
37 29 70 41
85 69 76 50
53 10 95 100",
            "250"
        ),
        (
            r"8 2
31000000 41000000
59000000 26000000
53000000 58000000
97000000 93000000
23000000 84000000
62000000 64000000
33000000 83000000
27000000 95000000",
            "581000000"
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
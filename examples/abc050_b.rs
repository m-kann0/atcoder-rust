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
    let t: Vec<usize> = (0..n).map(|_| iterator.next().unwrap().parse().unwrap()).collect();
    let m: usize = iterator.next().unwrap().parse().unwrap();
    let mut p = Vec::new();
    let mut x = Vec::new();
    for _ in 0..m {
        let pi: usize = iterator.next().unwrap().parse().unwrap();
        let xi: usize = iterator.next().unwrap().parse().unwrap();
        p.push(pi - 1);
        x.push(xi);
    }

    let total: usize = t.iter().sum();

    let mut result = String::new();
    for i in 0..m {
        let mut now = total;
        now -= t[p[i]];
        now += x[i];
        result.push_str(&format!("{}\n", now));
    }
    result
}

#[test]
fn test() {
    let cases: Vec<(&str, &str)> = vec![
        (
            r"3
2 1 4
2
1 1
2 3",
            "6
9"
        ),
        (
            r"5
7 2 3 8 5
3
4 2
1 7
4 13",
            "19
25
30"
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
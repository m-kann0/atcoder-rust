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
    let t: f64 = iterator.next().unwrap().parse().unwrap();
    let a: f64 = iterator.next().unwrap().parse().unwrap();
    let h: Vec<f64> = (0..n).map(|_| iterator.next().unwrap().parse().unwrap()).collect();

    let mut ans = 0;
    for i in 1..n {
        if (t - h[ans] * 0.006 - a).abs() > (t - h[i] * 0.006 - a).abs() {
            ans = i;
        }
    }
    (ans + 1).to_string()
}

#[test]
fn test() {
    let cases: Vec<(&str, &str)> = vec![
        (
            r"2
12 5
1000 2000",
            "1"
        ),
        (
            r"3
21 -11
81234 94124 52141",
            "3"
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
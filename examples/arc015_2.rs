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
    let mut counts = vec![0_usize; 6];
    for _ in 0..n {
        let mx: f64 = iterator.next().unwrap().parse().unwrap();
        let mn: f64 = iterator.next().unwrap().parse().unwrap();
        if mx >= 35.0 {
            counts[0] += 1;
        }
        if mx >= 30.0 && mx < 35.0 {
            counts[1] += 1;
        }
        if mx >= 25.0 && mx < 30.0 {
            counts[2] += 1;
        }
        if mn >= 25.0 {
            counts[3] += 1;
        }
        if mn < 0.0 && mx >= 0.0 {
            counts[4] += 1;
        }
        if mx < 0.0 {
            counts[5] += 1;
        }
    }

    let mut result = String::new();
    for i in 0..6 {
        result.push_str(&format!("{} ", counts[i]));
    }
    result.trim().to_string()
}

#[test]
fn test() {
    let cases: Vec<(&str, &str)> = vec![
        (
            r"4
32.2 25.3
36.4 26.4
24.1 18.0
26.0 24.9",
            "1 1 1 2 0 0"
        ),
        (
            r"3
-2 -5.2
2 -0.1
26.0 -0.1",
            "0 0 1 0 2 1"
        ),
        (
            r"4
15.0 9.5
12.5 10.5
20.5 19.9
15.5 15.5",
            "0 0 0 0 0 0"
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
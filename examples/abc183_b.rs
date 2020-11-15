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

    let x1: f64 = iterator.next().unwrap().parse().unwrap();
    let y1: f64 = iterator.next().unwrap().parse().unwrap();
    let x2: f64 = iterator.next().unwrap().parse().unwrap();
    let y2: f64 = iterator.next().unwrap().parse().unwrap();
    let y1 = -y1;

    let ans = (-y1) * (x2 - x1) / (y2 - y1) + x1;
    ans.to_string()
}

#[test]
fn test() {
    let cases: Vec<(&str, &str)> = vec![
        (
            r"1 1 7 2",
            "3.0000000000"
        ),
        (
            r"1 1 3 2",
            "1.6666666667"
        ),
        (
            r"-9 99 -999 9999",
            "-18.7058823529"
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

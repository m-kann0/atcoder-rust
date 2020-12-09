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

    let p: f64 = iterator.next().unwrap().parse().unwrap();

    fn f(p: f64, x: f64) -> f64 {
        x + p / 2_f64.powf(x / 1.5)
    }

    let mut low = 0.0;
    let mut high = 100.0;
    while high - low > 0.00000001 {
        let mid_low = high / 3.0 + low * 2.0 / 3.0;
        let mid_high = high * 2.0 / 3.0 + low / 3.0;
        if f(p, mid_low) >= f(p, mid_high) {
            low = mid_low;
        } else {
            high = mid_high
        }
    }
    f(p, high).to_string()
}

#[test]
fn test() {
    let cases: Vec<(&str, &str)> = vec![
        (
            r"3.0000",
            "2.8708930019"
        ),
        (
            r"0.0400",
            "0.0400000000"
        ),
        (
            r"1000000000000000000.0000",
            "90.1855078128"
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
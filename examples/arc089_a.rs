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
    let mut t = vec![0];
    let mut x = vec![0];
    let mut y = vec![0];
    for _ in 0..n {
        let ti: isize = iterator.next().unwrap().parse().unwrap();
        let xi: isize = iterator.next().unwrap().parse().unwrap();
        let yi: isize = iterator.next().unwrap().parse().unwrap();
        t.push(ti);
        x.push(xi);
        y.push(yi);
    }

    for i in 1..=n {
        if t[i] % 2 != (x[i] + y[i]) % 2 {
            return "No".to_string();
        }
        if (x[i] - x[i - 1]).abs() + (y[i] - y[i - 1]).abs() > t[i] - t[i - 1] {
            return "No".to_string();
        }
    }
    "Yes".to_string()
}

#[test]
fn test() {
    let cases: Vec<(&str, &str)> = vec![
        (
            r"2
3 1 2
6 1 1",
            "Yes"
        ),
        (
            r"1
2 100 100",
            "No"
        ),
        (
            r"2
5 1 1
100 1 1",
            "No"
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
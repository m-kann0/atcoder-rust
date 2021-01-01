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
    let mut t = Vec::new();
    let mut a = Vec::new();
    for _ in 0..n {
        let ti: usize = iterator.next().unwrap().parse().unwrap();
        let ai: usize = iterator.next().unwrap().parse().unwrap();
        t.push(ti);
        a.push(ai);
    }

    let mut tc = t[0];
    let mut ac = a[0];
    for i in 1..n {
        let x = (tc + t[i] - 1) / t[i];
        let y = (ac + a[i] - 1) / a[i];
        let k = max(x, y);
        tc = t[i] * k;
        ac = a[i] * k;
    }
    (tc + ac).to_string()
}

#[test]
fn test() {
    let cases: Vec<(&str, &str)> = vec![
        (
            r"3
2 3
1 1
3 2",
            "10"
        ),
        (
            r"4
1 1
1 1
1 5
1 100",
            "101"
        ),
        (
            r"5
3 10
48 17
31 199
231 23
3 2",
            "6930"
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
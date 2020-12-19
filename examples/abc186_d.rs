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
    let mut a: Vec<isize> = (0..n).map(|_| iterator.next().unwrap().parse().unwrap()).collect();
    a.sort();
    let mut ans: isize = 0;
    for i in 1..=n {
        ans += (i as isize - 1) * a[i - 1];
        ans -= (n as isize - i as isize) * a[i - 1];
    }
    ans.to_string()
}




#[test]
fn test() {
    let cases: Vec<(&str, &str)> = vec![
        (
            r"3
5 1 2",
            "8"
        ),
        (
            r"5
31 41 59 26 53",
            "176"
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
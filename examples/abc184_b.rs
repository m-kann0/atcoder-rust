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
    let mut x: isize = iterator.next().unwrap().parse().unwrap();
    let s: Vec<char> = iterator.next().unwrap().chars().collect();

    for i in 0..n {
        if s[i] == 'o' {
            x += 1;
        } else if x > 0 {
            x -= 1;
        }
    }
    x.to_string()
}

#[test]
fn test() {
    let cases: Vec<(&str, &str)> = vec![
        (
            r"3 0
xox",
            "0"
        ),
        (
            r"20 199999
oooooooooxoooooooooo",
            "200017"
        ),
        (
            r"20 10
xxxxxxxxxxxxxxxxxxxx",
            "0"
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
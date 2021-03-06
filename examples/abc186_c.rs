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
    let mut ans: usize = 0;
    for i in 1..=n {
        let mut is_ok = true;

        let mut x = i;
        while x > 0 {
            if x % 10 == 7 {
                is_ok = false;
                break;
            }
            x /= 10;
        }

        let mut x = i;
        while x > 0 {
            if x % 8 == 7 {
                is_ok = false;
                break;
            }
            x /= 8;
        }

        if is_ok {
            ans += 1;
        }
    }
    ans.to_string()
}

#[test]
fn test() {
    let cases: Vec<(&str, &str)> = vec![
        (
            r"20",
            "17"
        ),
        (
            r"100000",
            "30555"
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
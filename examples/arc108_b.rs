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
    let s: Vec<char> = iterator.next().unwrap().chars().collect();

    let mut stack = Vec::new();
    for i in 0..n {
        if s[i] == 'x' {
            if stack.len() >= 2 {
                let l1 = stack.pop().unwrap();
                let l2 = stack.pop().unwrap();
                if l2 == 'f' && l1 == 'o' {
                    // do nothing
                } else {
                    stack.push(l2);
                    stack.push(l1);
                    stack.push(s[i]);
                }
            } else {
                stack.push(s[i]);
            }
        } else {
            stack.push(s[i]);
        }
    }
    stack.len().to_string()
}

#[test]
fn test() {
    let cases: Vec<(&str, &str)> = vec![
        (
            r"6
icefox",
            "3"
        ),
        (
            r"7
firebox",
            "7"
        ),
        (
            r"48
ffoxoxuvgjyzmehmopfohrupffoxoxfofofoxffoxoxejffo",
            "27"
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
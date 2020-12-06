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
    let s: Vec<u32> = s.iter().map(|c| c.to_digit(10).unwrap()).collect();

    let mut ans = 0;
    for i in 0..=9 {
        for j in 0..=9 {
            for k in 0..=9 {
                let mut si = 0;
                while si < n {
                    if s[si] == i {
                        break;
                    }
                    si += 1;
                }
                si += 1;
                if si >= n {
                    continue;
                }
                while si < n {
                    if s[si] == j {
                        break;
                    }
                    si += 1;
                }
                si += 1;
                if si >= n {
                    continue;
                }
                while si < n {
                    if s[si] == k {
                        break;
                    }
                    si += 1;
                }
                if si == n {
                    continue;
                }
                ans += 1;
            }
        }
    }
    ans.to_string()
}

#[test]
fn test() {
    let cases: Vec<(&str, &str)> = vec![
        (
            r"4
0224",
            "3"
        ),
        (
            r"6
123123",
            "17"
        ),
        (
            r"19
3141592653589793238",
            "329"
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
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
    let mut t: Vec<char> = iterator.next().unwrap().chars().collect();


    if n == 1 {
        return if t[0] == '0' {
            10_000_000_000_usize.to_string()
        } else {
            20_000_000_000_usize.to_string()
        }
    }

    if t[0] == '0' {
        t.insert(0, '1');
        t.insert(0, '1');
    } else if t[0] == '1' && t[1] == '0' {
        t.insert(0, '1');
    }
    if t[t.len() - 1] == '1' {
        if t[t.len() - 2] == '1' {
            t.push('0');
        } else {
            t.push('1');
            t.push('0');
        }
    }

    if t.len() % 3 != 0 {
        return "0".to_string();
    }

    let mut ans = 10_000_000_001_usize;
    let mut i = 0;
    while i < t.len() {
        if t[i] != '1' || t[i + 1] != '1' || t[i + 2] != '0' {
            return "0".to_string();
        }
        i += 3;
        ans -= 1;
    }
    ans.to_string()
}

#[test]
fn test() {
    let cases: Vec<(&str, &str)> = vec![
        (
            r"4
1011",
            "9999999999"
        ),
        (
            r"22
1011011011011011011011",
            "9999999993"
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
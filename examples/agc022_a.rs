use std::io::Read;
use std::collections::HashMap;

fn main() {
    let mut buf = String::new();
    std::io::stdin().read_to_string(&mut buf).unwrap();

    let answer = solve(&buf);

    println!("{}", answer);
}

fn solve(input: &str) -> String {
    let mut iterator = input.split_whitespace();

    let s = iterator.next().unwrap();

    if s == "zyxwvutsrqponmlkjihgfedcba" {
        return "-1".to_string();
    }

    let alphabets: Vec<char> = "abcdefghijklmnopqrstuvwxyz".chars().collect();

    let mut counts: HashMap<char, usize> = HashMap::new();
    for c in s.chars() {
        *counts.entry(c).or_insert(0) += 1;
    }

    if s.len() < 26 {
        let mut next_c = ' ';
        for alphabet in alphabets {
            if !counts.contains_key(&alphabet) {
                next_c = alphabet;
                break;
            }
        }
        return format!("{}{}", s, next_c);
    }

    let s_chars: Vec<char> = s.chars().collect();
    let mut position = 0;
    for i in (1..26).rev() {
        let ca = s_chars[i - 1];
        let cb = s_chars[i];
        let ca_idx = alphabets.iter().position(|it| *it == ca).unwrap();
        let cb_idx = alphabets.iter().position(|it| *it == cb).unwrap();
        if ca_idx > cb_idx {
            continue;
        } else {
            position = i;
            break;
        }
    }

    let part1 = &s[..(position - 1)];

    let q = *&s[..(position)].chars().last().unwrap();

    let mut others: Vec<char> = (&s[position..]).chars().collect();
    others.sort();

    let mut next_c = ' ';
    for c in others {
        let q_idx = alphabets.iter().position(|it| *it == q).unwrap();
        let c_idx = alphabets.iter().position(|it| *it == c).unwrap();
        if q_idx < c_idx {
            next_c = c;
            break;
        }
    }

    return format!("{}{}", part1, next_c);
}

#[test]
fn test() {
    let cases: Vec<(&str, &str)> = vec![
        (
            r"atcoder",
            "atcoderb"
        ),
        (
            r"abc",
            "abcd"
        ),
        (
            r"zyxwvutsrqponmlkjihgfedcba",
            "-1"
        ),
        (
            r"abcdefghijklmnopqrstuvwzyx",
            "abcdefghijklmnopqrstuvx"
        ),
        (
            r"yzxwvutsrqponmlkjihgfedcba",
            "z"
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
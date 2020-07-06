use std::io::Read;
use std::collections::HashSet;

fn main() {
    let mut buf = String::new();
    std::io::stdin().read_to_string(&mut buf).unwrap();

    let answer = solve(&buf);

    println!("{}", answer);
}

fn solve(input: &str) -> String {
    let mut iterator = input.split_whitespace();

    let cards: String = iterator.next().unwrap().to_string();

    let mut result = cards.clone();
    let marks = vec!['S', 'H', 'D', 'C'];
    for mark in marks {
        if can_make(&cards, mark) {
            let r = make(&cards, mark);
            if r.len() < result.len() {
                result = r;
            }
        }
    }
    return result;
}

fn can_make(cards: &String, mark: char) -> bool {
    let numbers: Vec<&str> = vec!["10", "J", "Q", "K", "A"];
    for number in numbers {
        let target = format!("{}{}", mark, number);
        if !cards.contains(&target) {
            return false;
        }
    }
    true
}

fn make(cards: &String, mark: char) -> String {
    let mut result = String::new();
    let mut cards = cards.clone();
    let mut set: HashSet<String> = HashSet::new();
    while set.len() < 5 && cards.len() > 0 {
        if cards.starts_with(&format!("{}{}", mark, "10")) {
            if !set.contains(&format!("{}{}", mark, "10")) {
                set.insert(format!("{}{}", mark, "10"));
                cards = cards.replacen(&format!("{}{}", mark, "10"), "", 1);
            } else {
                result.push_str(&format!("{}{}", mark, "10"));
                cards = cards.replacen(&format!("{}{}", mark, "10"), "", 1);
            }
        } else if cards.starts_with(&format!("{}{}", mark, "J")) {
            if !set.contains(&format!("{}{}", mark, "J")) {
                set.insert(format!("{}{}", mark, "J"));
                cards = cards.replacen(&format!("{}{}", mark, "J"), "", 1);
            } else {
                result.push_str(&format!("{}{}", mark, "J"));
                cards = cards.replacen(&format!("{}{}", mark, "J"), "", 1);
            }
        } else if cards.starts_with(&format!("{}{}", mark, "Q")) {
            if !set.contains(&format!("{}{}", mark, "Q")) {
                set.insert(format!("{}{}", mark, "Q"));
                cards = cards.replacen(&format!("{}{}", mark, "Q"), "", 1);
            } else {
                result.push_str(&format!("{}{}", mark, "Q"));
                cards = cards.replacen(&format!("{}{}", mark, "Q"), "", 1);
            }
        } else if cards.starts_with(&format!("{}{}", mark, "K")) {
            if !set.contains(&format!("{}{}", mark, "K")) {
                set.insert(format!("{}{}", mark, "K"));
                cards = cards.replacen(&format!("{}{}", mark, "K"), "", 1);
            } else {
                result.push_str(&format!("{}{}", mark, "K"));
                cards = cards.replacen(&format!("{}{}", mark, "K"), "", 1);
            }
        } else if cards.starts_with(&format!("{}{}", mark, "A")) {
            if !set.contains(&format!("{}{}", mark, "A")) {
                set.insert(format!("{}{}", mark, "A"));
                cards = cards.replacen(&format!("{}{}", mark, "A"), "", 1);
            } else {
                result.push_str(&format!("{}{}", mark, "A"));
                cards = cards.replacen(&format!("{}{}", mark, "A"), "", 1);
            }
        } else {
            result.push(cards.remove(0));
            while !cards.starts_with("S") && !cards.starts_with("H")
                && !cards.starts_with("D") && !cards.starts_with("C") {
                if cards.len() == 0 {
                    break;
                }
                result.push(cards.remove(0));
            }
        }
    }
    if result.is_empty() {
        "0".to_string()
    } else {
        result
    }
}

#[test]
fn test() {
    let cases: Vec<(&str, &str)> = vec![
        (
            r"CQSAS10SQH10SKSJD3",
            r"CQH10"
        ),
        (
            r"S10SJSQSKSAC2",
            r"0"
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
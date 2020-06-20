use std::io::Read;

fn main() {
    let mut buf = String::new();
    std::io::stdin().read_to_string(&mut buf).unwrap();

    let answer = solve(&buf);

    println!("{}", answer);
}

fn solve(input: &str) -> String {
    let mut iterator = input.split_whitespace();

    let s: Vec<char> = iterator.next().unwrap().chars().collect();
    let mut counts = std::collections::HashMap::new();
    for i in 0..4 {
        *counts.entry(s[i]).or_insert(0) += 1;
    }

    if counts.len() == 2 && counts.iter().all(|e| *e.1 == 2) {
        return "Yes".to_string();
    }

    return "No".to_string();
}

#[test]
fn test() {
    let cases: Vec<(&str, &str)> = vec![
        (
            r"ASSA",
            "Yes"
        ),
        (
            r"STOP",
            "No"
        ),
        (
            r"FFEE",
            "Yes"
        ),
        (
            r"FREE",
            "No"
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
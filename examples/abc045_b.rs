use std::io::Read;
use std::collections::VecDeque;

fn main() {
    let mut buf = String::new();
    std::io::stdin().read_to_string(&mut buf).unwrap();

    let answer = solve(&buf);

    println!("{}", answer);
}

fn solve(input: &str) -> String {
    let mut iterator = input.split_whitespace();

    let mut sa: VecDeque<char> = iterator.next().unwrap().chars().collect();
    let mut sb: VecDeque<char> = iterator.next().unwrap().chars().collect();
    let mut sc: VecDeque<char> = iterator.next().unwrap().chars().collect();

    let mut next: char = sa.pop_front().unwrap();
    loop {
        if next == 'a' {
            if sa.is_empty() {
                return "A".to_string();
            } else {
                next = sa.pop_front().unwrap();
            }
        } else if next == 'b' {
            if sb.is_empty() {
                return "B".to_string();
            } else {
                next = sb.pop_front().unwrap();
            }
        } else {
            if sc.is_empty() {
                return "C".to_string();
            } else {
                next = sc.pop_front().unwrap();
            }
        }
    }
}

#[test]
fn test() {
    let cases: Vec<(&str, &str)> = vec![
        (
            r"aca
accc
ca",
            "A"
        ),
        (
            r"abcb
aacb
bccc",
            "C"
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
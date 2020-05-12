use std::io::Read;

fn main() {
    let mut buf = String::new();
    std::io::stdin().read_to_string(&mut buf).unwrap();

    let answer = solve(&buf);

    println!("{}", answer);
}

fn solve(input: &str) -> String {
    let mut iterator = input.split_whitespace();

    let mut s: String = iterator.next().unwrap().to_string();

    let patterns: Vec<&str> = vec!["dream", "dreamer", "erase", "eraser"];

    while !s.is_empty() {
        let result = patterns.iter().find(|it| s.ends_with(**it));
        if let Some(x) = result {
            let len = s.len();
            s.truncate(len - x.len());
        } else {
            break;
        }
    }

    if s.is_empty() {
        return "YES".to_string();
    }

    return "NO".to_string()
}

#[test]
fn test() {
    let cases: Vec<(&str, &str)> = vec![
        (
            r"erasedream",
            "YES"
        ),
        (
            r"dreameraser",
            "YES"
        ),
        (
            r"dreamerer",
            "NO"
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
use std::io::Read;

fn main() {
    let mut buf = String::new();
    std::io::stdin().read_to_string(&mut buf).unwrap();

    let answer = solve(&buf);

    println!("{}", answer);
}

fn solve(input: &str) -> String {
    let mut iterator = input.split_whitespace();

    let mut s: &str = iterator.next().unwrap();

    while !s.is_empty() {
        if s.starts_with("eraser") {
            s = &s[6..];
        } else if s.starts_with("erase") {
            s = &s[5..];
        } else if s.starts_with("dreamerd") || s.starts_with("dreamere") || s == "dreamer" {
            s = &s[7..];
        } else if s.starts_with("dream") {
            s = &s[5..];
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
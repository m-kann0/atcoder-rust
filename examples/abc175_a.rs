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

    if s[0] == 'R' && s[0] == s[1] && s[1] == s[2] {
        return "3".to_string();
    }

    if s[0] == 'R' && s[0] == s[1] || s[1] == 'R' && s[1] == s[2] {
        return "2".to_string();
    }

    if s[0] == 'R' || s[1] == 'R' || s[2] == 'R' {
        return "1".to_string();
    }
    "0".to_string()
}

#[test]
fn test() {
    let cases: Vec<(&str, &str)> = vec![
        (
            r"RRS",
            "2"
        ),
        (
            r"SSS",
            "0"
        ),
        (
            r"RSR",
            "1"
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
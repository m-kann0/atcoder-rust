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

    for i in 0..s.len() {
        if i % 2 == 0 {
            if s[i] == 'L' {
                return "No".to_string();
            }
        } else {
            if s[i] == 'R' {
                return "No".to_string();
            }
        }
    }

    return "Yes".to_string();
}

#[test]
fn test() {
    let cases: Vec<(&str, &str)> = vec![
        (
            r"RUDLUDR",
            "Yes"
        ),
        (
            r"DULL",
            "No"
        ),
        (
            r"UUUUUUUUUUUUUUU",
            "Yes"
        ),
        (
            r"ULURU",
            "No"
        ),
        (
            r"RDULULDURURLRDULRLR",
            "Yes"
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
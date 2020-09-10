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

    if s[0] == '{' && s[1] == '}' {
        return "dict".to_string();
    }

    let mut level = 0;
    for i in 0..s.len() {
        match s[i] {
            '{' => {
                level += 1;
            },
            '}' => {
                level -= 1;
            },
            ':' => {
                if level == 1 {
                    return "dict".to_string();
                }
            },
            ',' => {
                if level == 1 {
                    return "set".to_string();
                }
            }
            _ => {}
        }
    }

    "set".to_string()
}

#[test]
fn test() {
    let cases: Vec<(&str, &str)> = vec![
        (
            r"{1:2}",
            "dict"
        ),
        (
            r"{1,2}",
            "set"
        ),
        (
            r"{}",
            "dict"
        ),
        (
            r"{{123}:123}",
            "dict"
        ),
        (
            r"{{123:456},{123:456}}",
            "set"
        ),
        (
            r"{{123,456}:{123,456}}",
            "dict"
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
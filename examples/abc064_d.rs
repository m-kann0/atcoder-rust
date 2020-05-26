use std::io::Read;

fn main() {
    let mut buf = String::new();
    std::io::stdin().read_to_string(&mut buf).unwrap();

    let answer = solve(&buf);

    println!("{}", answer);
}

fn solve(input: &str) -> String {
    let mut iterator = input.split_whitespace();

    let _n: usize = iterator.next().unwrap().parse().unwrap();
    let s: &str = iterator.next().unwrap();

    let mut head = String::new();

    let mut stack: Vec<char> = Vec::new();
    for c in s.chars() {
        if c == '(' {
            stack.push(c);
        } else {
            if let Some(_) = stack.pop() {
                // nothing
            } else {
                head.push('(');
            }
        }
    }

    let mut tail = String::new();
    for _ in 0..stack.len() {
        tail.push(')');
    }

    return format!("{}{}{}", head, s, tail);
}

#[test]
fn test() {
    let cases: Vec<(&str, &str)> = vec![
        (
            r"3
())",
            "(())"
        ),
        (
            r"6
)))())",
            "(((()))())"
        ),
        (
            r"8
))))((((",
            "(((())))(((())))"
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
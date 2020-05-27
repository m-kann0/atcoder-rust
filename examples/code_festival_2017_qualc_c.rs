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

    let mut s: VecDeque<char> = iterator.next().unwrap().chars().collect();

    let mut ans = 0;
    while s.len() >= 2 {
        let f = s.front().unwrap();
        let b = s.back().unwrap();
        if f == b {
            s.pop_front().unwrap();
            s.pop_back().unwrap();
        } else if *f == 'x' {
            s.push_back('x');
            ans += 1;
        } else if *b == 'x' {
            s.push_front('x');
            ans += 1;
        } else {
            return "-1".to_string();
        }
    }

    return ans.to_string();
}

#[test]
fn test() {
    let cases: Vec<(&str, &str)> = vec![
        (
            r"xabxa",
            "2"
        ),
        (
            r"ab",
            "-1"
        ),
        (
            r"a",
            "0"
        ),
        (
            r"oxxx",
            "3"
        ),
        (
            r"xxx",
            "0"
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
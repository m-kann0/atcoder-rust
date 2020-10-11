use std::io::Read;
use std::cmp::max;

fn main() {
    let mut buf = String::new();
    std::io::stdin().read_to_string(&mut buf).unwrap();

    let answer = solve(&buf);

    println!("{}", answer);
}

fn solve(input: &str) -> String {
    let mut iterator = input.split_whitespace();

    let s: Vec<char> = iterator.next().unwrap().chars().collect();

    let mut ans = 0;
    let mut count = 0;
    for i in 0..s.len() {
        if s[i] == 'A' || s[i] == 'T' || s[i] == 'G' || s[i] == 'C' {
            count += 1;
            ans = max(ans, count);
        } else {
            count = 0;
        }
    }
    ans = max(ans, count);
    ans.to_string()
}

#[test]
fn test() {
    let cases: Vec<(&str, &str)> = vec![
        (
            r"ATCODER",
            "3"
        ),
        (
            r"HATAGAYA",
            "5"
        ),
        (
            r"SHINJUKU",
            "0"
        ),
        (
            r"ODERATC",
            "3"
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
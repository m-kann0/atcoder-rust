use std::io::Read;
use std::cmp::min;

fn main() {
    let mut buf = String::new();
    std::io::stdin().read_to_string(&mut buf).unwrap();

    let answer = solve(&buf);

    println!("{}", answer);
}

fn solve(input: &str) -> String {
    let mut iterator = input.split_whitespace();

    let s: Vec<char> = iterator.next().unwrap().chars().collect();

    let mut ans = 100;
    for c in 'a' as u8 .. 'z' as u8 + 1 {
        let c = c as char;
        if !s.contains(&c) {
            continue;
        }

        let mut t = s.clone();
        let mut count = 0;
        while !t.iter().all(|it| *it == c) {
            for i in 0 .. t.len() - 1 {
                if t[i] == c || t[i + 1] == c {
                    t[i] = c;
                }
            }
            t.pop();
            count += 1;
        }
        ans = min(ans, count);
    }

    return ans.to_string();
}

#[test]
fn test() {
    let cases: Vec<(&str, &str)> = vec![
        (
            r"serval",
            "3"
        ),
        (
            r"jackal",
            "2"
        ),
        (
            r"zzz",
            "0"
        ),
        (
            r"whbrjpjyhsrywlqjxdbrbaomnw", // 26
            "8"
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
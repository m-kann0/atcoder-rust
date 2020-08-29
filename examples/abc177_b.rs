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
    let t: Vec<char> = iterator.next().unwrap().chars().collect();

    let mut ans = t.len();
    for i in 0..=(s.len() - t.len()) {
        let mut a = 0;
        for j in 0..t.len() {
            if s[i + j] != t[j] {
                a += 1;
            }
        }
        ans = min(ans, a);
    }
    ans.to_string()
}

#[test]
fn test() {
    let cases: Vec<(&str, &str)> = vec![
        (
            r"cabacc
abc",
            "1"
        ),
        (
            r"cabacc
acc",
            "0"
        ),
        (
            r"codeforces
atcoder",
            "6"
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
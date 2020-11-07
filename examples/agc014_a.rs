use std::io::Read;
use std::collections::HashSet;
use std::cmp::{min, max};

fn main() {
    let mut buf = String::new();
    std::io::stdin().read_to_string(&mut buf).unwrap();

    let answer = solve(&buf);

    println!("{}", answer);
}

fn solve(input: &str) -> String {
    let mut iterator = input.split_whitespace();

    let mut a: usize = iterator.next().unwrap().parse().unwrap();
    let mut b: usize = iterator.next().unwrap().parse().unwrap();
    let mut c: usize = iterator.next().unwrap().parse().unwrap();

    let mut history: HashSet<(usize, usize, usize)> = HashSet::new();
    history.insert(key(a, b, c));
    let mut ans: usize = 0;
    while a % 2 == 0 && b % 2 == 0 && c % 2 == 0 {
        let na = b / 2 + c / 2;
        let nb = a / 2 + c / 2;
        let nc = a / 2 + b / 2;
        let key = (na, nb, nc);
        if history.contains(&key) {
            return "-1".to_string();
        }
        history.insert(key);
        a = na;
        b = nb;
        c = nc;
        ans += 1;
    }
    ans.to_string()
}

fn key(a: usize, b: usize, c: usize) -> (usize, usize, usize) {
    let mn = min(a, min(b, c));
    let mx = max(a, max(b, c));
    let md = a + b + c - mn - mx;
    let key = (mn, md, mx);
    key
}

#[test]
fn test() {
    let cases: Vec<(&str, &str)> = vec![
        (
            r"4 12 20",
            "3"
        ),
        (
            r"14 14 14",
            "-1"
        ),
        (
            r"454 414 444",
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
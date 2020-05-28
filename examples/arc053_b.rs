use std::io::Read;
use std::collections::HashMap;

fn main() {
    let mut buf = String::new();
    std::io::stdin().read_to_string(&mut buf).unwrap();

    let answer = solve(&buf);

    println!("{}", answer);
}

fn solve(input: &str) -> String {
    let mut iterator = input.split_whitespace();

    let s: Vec<char> = iterator.next().unwrap().chars().collect();

    let all: usize = s.len();

    let mut counts: HashMap<char, usize> = HashMap::new();
    for c in &s {
        *counts.entry(*c).or_insert(0) += 1;
    }

    let odd: usize = counts.iter().filter(|&(_k, v)| *v % 2 == 1).count();

    if odd <= 1 {
        return all.to_string();
    }

    let rest: usize = all - odd;
    let ans = rest / 2 / odd * 2 + 1;
    return ans.to_string();
}

#[test]
fn test() {
    let cases: Vec<(&str, &str)> = vec![
        (r"rokovoko", r"3"),
        (r"tomtom", r"6"),
        (r"vwxyz", r"1"),
        (r"succeeded", r"3"),
        (r"succeededaaaa", r"3"),
        (r"succeededaaaaaa", r"5"),
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
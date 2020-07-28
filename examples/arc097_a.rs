use std::io::Read;
use std::collections::HashSet;
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
    let k: usize = iterator.next().unwrap().parse().unwrap();

    let mut substrings: HashSet<String> = HashSet::new();
    for i in 0..s.len() {
        let mut substring = String::new();
        for j in i..min(s.len(), i + k) {
            substring.push(s[j]);
            substrings.insert(substring.clone());
        }
    }
    let mut substrings: Vec<String> = substrings.clone().into_iter().collect();
    substrings.sort();
    substrings[k - 1].clone()
}

#[test]
fn test() {
    let cases: Vec<(&str, &str)> = vec![
        (
            r"aba
4",
            "b"
        ),
        (
            r"atcoderandatcodeer
5",
            "andat"
        ),
        (
            r"z
1",
            "z"
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
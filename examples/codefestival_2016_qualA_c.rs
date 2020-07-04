use std::io::Read;
use itertools::Itertools;

fn main() {
    let mut buf = String::new();
    std::io::stdin().read_to_string(&mut buf).unwrap();

    let answer = solve(&buf);

    println!("{}", answer);
}

fn solve(input: &str) -> String {
    let mut iterator = input.split_whitespace();

    let s: Vec<char> = iterator.next().unwrap().chars().collect();
    let mut k: usize = iterator.next().unwrap().parse().unwrap();

    let costs: Vec<usize> = s.iter()
        .map(|c| ('z' as usize - *c as usize + 1) % 26)
        .collect();

    let mut result: Vec<char> = Vec::new();
    for i in 0..s.len() {
        if costs[i] <= k {
            result.push('a');
            k -= costs[i];
        } else {
            result.push(s[i]);
        }
    }

    if k > 0 {
        result[s.len() - 1] = ('a' as usize + (result[s.len() - 1] as usize - 'a' as usize + k) % 26) as u8 as char
    }

    return result.iter().join("")
}

#[test]
fn test() {
    let cases: Vec<(&str, &str)> = vec![
        (
            r"xyz
4",
            "aya"
        ),
        (
            r"a
25",
            "z"
        ),
        (
            r"codefestival
100",
            "aaaafeaaivap"
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
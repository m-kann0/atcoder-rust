use std::io::Read;
use std::collections::HashMap;
use std::cmp::max;

fn main() {
    let mut buf = String::new();
    std::io::stdin().read_to_string(&mut buf).unwrap();

    let answer = solve(&buf);

    println!("{}", answer);
}

fn solve(input: &str) -> String {
    let mut iterator = input.split_whitespace();

    let _: usize = iterator.next().unwrap().parse().unwrap();
    let _: usize = iterator.next().unwrap().parse().unwrap();

    let name: Vec<char> = iterator.next().unwrap().chars().collect();
    let mut counts_n: HashMap<char, usize> = HashMap::new();
    for &c in name.iter() {
        *counts_n.entry(c).or_insert(0) += 1;
    }

    let kit: Vec<char> = iterator.next().unwrap().chars().collect();
    let mut counts_k: HashMap<char, usize> = HashMap::new();
    for &c in kit.iter() {
        *counts_k.entry(c).or_insert(0) += 1;
    }

    let mut ans = 1;
    for (c, count) in counts_n.iter() {
        if let Some(count_k) = counts_k.get(c) {
            if count > count_k {
                ans = max(ans, (count + count_k - 1) / count_k);
            }
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
            r"7 26
NAOHIRO
ABCDEFGHIJKLMNOPQRSTUVWXYZ",
            r"2"
        ),
        (
            r"8 8
TAKOYAKI
TAKOYAKI",
            r"1"
        ),
        (
            r"8 4
CHOKUDAI
MYON",
            r"-1"
        ),
        (
            r"6 6
MONAKA
NAMAKO",
            r"1"
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
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

    let n: usize = iterator.next().unwrap().parse().unwrap();
    let mut counts: HashMap<usize, usize> = HashMap::new();
    for _ in 0..n {
        let ai: usize = iterator.next().unwrap().parse().unwrap();
        *counts.entry(ai).or_insert(0) += 1;
    }

    if n % 2 == 0 {
        for i in 0..(n / 2) {
            let ai = 2 * i + 1;
            if !counts.contains_key(&ai) || *counts.get(&ai).unwrap() != 2 {
                return "0".to_string();
            }
        }
        return mod_pow(2, n / 2).to_string();
    }

    let zero = 0;
    if !counts.contains_key(&zero) || *counts.get(&zero).unwrap() != 1 {
        return "0".to_string();
    }
    for i in 0..(n / 2) {
        let ai = 2 * (i + 1);
        if !counts.contains_key(&ai) || *counts.get(&ai).unwrap() != 2 {
            return "0".to_string();
        }
    }

    return mod_pow(2, n / 2).to_string();
}

const MOD: usize = 1000000007;

fn mod_pow(a: usize, b: usize) -> usize {
    let mut result = 1;
    for _ in 0..b {
        result = result * a % MOD;
    }
    return result
}

#[test]
fn test() {
    let cases: Vec<(&str, &str)> = vec![
        (
            r"5
2 4 4 0 2",
            "4"
        ),
        (
            r"7
6 4 0 2 4 0 2",
            "0"
        ),
        (
            r"8
7 5 1 1 7 3 5 3",
            "16"
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
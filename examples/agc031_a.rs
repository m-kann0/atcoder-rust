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
    let s: Vec<char> = iterator.next().unwrap().chars().collect();

    let mut counts: HashMap<char, usize> = HashMap::new();
    for i in 0..n {
        *counts.entry(s[i]).or_insert(0) += 1;
    }

    const MOD: usize = 1_000_000_007;
    let mut ans = 1;
    for (_, count) in counts {
        ans = ans * (count + 1) % MOD;
    }
    if ans == 0 {
        ans = MOD - 1;
    } else {
        ans = ans - 1;
    }
    return ans.to_string();
}

#[test]
fn test() {
    let cases: Vec<(&str, &str)> = vec![
        (
            r"4
abcd",
            "15"
        ),
        (
            r"3
baa",
            "5"
        ),
        (
            r"5
abcab",
            "17"
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
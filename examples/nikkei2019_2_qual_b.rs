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
    let d: Vec<usize> = (0..n)
        .map(|_| iterator.next().unwrap().parse().unwrap())
        .collect();
    if d[0] != 0 {
        return "0".to_string();
    }

    let mut counts: HashMap<usize, usize> = HashMap::new();
    for i in 0..n {
        *counts.entry(d[i]).or_insert(0) += 1;
    }

    if !counts.contains_key(&0) || *counts.get(&0).unwrap() != 1 {
        return "0".to_string();
    }

    let mut ans: usize = 1;
    let d_max: usize = *d.iter().max().unwrap();
    for i in 1..(d_max + 1) {
        if !counts.contains_key(&i) {
            return "0".to_string();
        }
        let a = counts.get(&(i - 1)).unwrap();
        let b = counts.get(&i).unwrap();
        ans = ans * mod_pow(*a, *b) % MOD
    }
    return ans.to_string();
}

const MOD: usize = 998244353;

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
            r"4
0 1 1 2",
            "2"
        ),
        (
            r"4
1 1 1 1",
            "0"
        ),
        (
            r"7
0 3 2 1 2 2 1",
            "24"
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
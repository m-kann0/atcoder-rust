use std::collections::HashMap;

use std::io::Read;

fn main() {
    let mut buf = String::new();
    std::io::stdin().read_to_string(&mut buf).unwrap();

    let answer = solve(&buf);

    println!("{}", answer);
}

fn solve(input: &str) -> String {
    let mut iterator = input.split_whitespace();

    let n: u64 = iterator.next().unwrap().parse().unwrap();
    let p: u64 = iterator.next().unwrap().parse().unwrap();

    if n == 1 {
        return p.to_string()
    }

    let mut counts: HashMap<u64, u64> = HashMap::new();

    let mut i = 2;
    while i * i <= p {
        if !is_prime(i) {
            i += 1;
            continue;
        }
        let mut x = p;
        while x % i == 0 {
            x /= i;
            *counts.entry(i).or_insert(0) += 1;
        }
        i += 1;
    }

    let mut g: u64 = 1;
    for (k, v) in counts {
        g *= k.pow((v / n) as u32);
    }
    return g.to_string();
}

fn is_prime(n: u64) -> bool {
    let mut i: u64 = 2;
    while i * i <= n {
        if n % i == 0 {
            return false;
        }
        i += 1;
    }
    return true;
}

#[test]
fn test() {
    let cases: Vec<(&str, &str)> = vec![
        (
            r"3 24",
            "2"
        ),
        (
            r"5 1",
            "1"
        ),
        (
            r"1 111",
            "111"
        ),
        (
            r"4 972439611840",
            "206"
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
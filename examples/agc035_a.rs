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
    let a: Vec<usize> = (0..n)
        .map(|_| iterator.next().unwrap().parse().unwrap())
        .collect();

    if n % 3 != 0 {
        return if a.iter().all(|it| *it == 0) {
            "Yes".to_string()
        } else {
            "No".to_string()
        }
    }

    let mut counts: HashMap<usize, usize> = HashMap::new();
    for ai in a {
        *counts.entry(ai).or_insert(0) += 1;
    }

    let len = counts.len();
    return if len == 1 {
        if counts.contains_key(&0) {
            "Yes".to_string()
        } else {
            "No".to_string()
        }
    } else if len == 2 {
        if !counts.contains_key(&0) {
            return "No".to_string();
        }

        let count_zero = counts.get(&0).unwrap();
        if n / 3 == *count_zero {
            "Yes".to_string()
        } else {
            "No".to_string()
        }
    } else if len == 3 {
        let vec: Vec<usize> = counts.keys().map(|it| *it).collect();
        let a = vec[0];
        let b = vec[1];
        let c = vec[2];
        if *counts.get(&a).unwrap() == n / 3
            && *counts.get(&b).unwrap() == n / 3
            && *counts.get(&c).unwrap() == n / 3
            && a ^ b ^ c == 0 {
            "Yes".to_string()
        } else {
            "No".to_string()
        }
    } else {
        "No".to_string()
    }
}

#[test]
fn test() {
    let cases: Vec<(&str, &str)> = vec![
        (
            r"3
1 2 3",
            "Yes"
        ),
        (
            r"4
1 2 4 8",
            "No"
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
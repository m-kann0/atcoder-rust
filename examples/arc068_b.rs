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
        let a: usize = iterator.next().unwrap().parse().unwrap();
        *counts.entry(a).or_insert(0) += 1;
    }

    let x: usize = counts.values().map(|it| *it - 1).sum();

    if x % 2 == 0 {
        (n - x).to_string()
    } else {
        (n - x - 1).to_string()
    }
}

#[test]
fn test() {
    let cases: Vec<(&str, &str)> = vec![
        (
            r"5
1 2 1 3 7",
            "3"
        ),
        (
            r"15
1 3 5 2 1 3 2 8 8 6 2 6 11 1 1",
            "7"
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
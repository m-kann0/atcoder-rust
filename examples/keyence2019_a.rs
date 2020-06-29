use std::io::Read;
use std::collections::HashSet;

fn main() {
    let mut buf = String::new();
    std::io::stdin().read_to_string(&mut buf).unwrap();

    let answer = solve(&buf);

    println!("{}", answer);
}

fn solve(input: &str) -> String {
    let mut iterator = input.split_whitespace();

    let s: HashSet<usize> = (0..4).map(|_| iterator.next().unwrap().parse().unwrap()).collect();

    return if s.contains(&1) && s.contains(&9) && s.contains(&7) && s.contains(&4) {
        "YES".to_string()
    } else {
        "NO".to_string()
    }
}

#[test]
fn test() {
    let cases: Vec<(&str, &str)> = vec![
        (
            r"1 7 9 4",
            "YES"
        ),
        (
            r"1 9 7 4",
            "YES"
        ),
        (
            r"1 2 9 1",
            "NO"
        ),
        (
            r"4 9 0 8",
            "NO"
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
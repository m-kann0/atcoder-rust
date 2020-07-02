use std::io::Read;

fn main() {
    let mut buf = String::new();
    std::io::stdin().read_to_string(&mut buf).unwrap();

    let answer = solve(&buf);

    println!("{}", answer);
}

fn solve(input: &str) -> String {
    let mut iterator = input.split_whitespace();

    let a = iterator.next().unwrap();
    let b = iterator.next().unwrap();
    let c = iterator.next().unwrap();

    if a.chars().rev().next().unwrap() == b.chars().next().unwrap()
        && b.chars().rev().next().unwrap() == c.chars().next().unwrap() {
        "YES".to_string()
    } else {
        "NO".to_string()
    }
}

#[test]
fn test() {
    let cases: Vec<(&str, &str)> = vec![
        (
            r"rng gorilla apple",
            "YES"
        ),
        (
            r"yakiniku unagi sushi",
            "NO"
        ),
        (
            r"a a a",
            "YES"
        ),
        (
            r"aaaaaaaaab aaaaaaaaaa aaaaaaaaab",
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
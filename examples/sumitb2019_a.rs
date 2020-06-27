use std::io::Read;

fn main() {
    let mut buf = String::new();
    std::io::stdin().read_to_string(&mut buf).unwrap();

    let answer = solve(&buf);

    println!("{}", answer);
}

fn solve(input: &str) -> String {
    let mut iterator = input.split_whitespace();

    let m1: usize = iterator.next().unwrap().parse().unwrap();
    let _d1: usize = iterator.next().unwrap().parse().unwrap();
    let m2: usize = iterator.next().unwrap().parse().unwrap();
    let _d2: usize = iterator.next().unwrap().parse().unwrap();

    return if m1 != m2 {
        "1".to_string()
    } else {
        "0".to_string()
    };
}

#[test]
fn test() {
    let cases: Vec<(&str, &str)> = vec![
        (
            r"11 16
11 17",
            "0"
        ),
        (
            r"11 30
12 1",
            "1"
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
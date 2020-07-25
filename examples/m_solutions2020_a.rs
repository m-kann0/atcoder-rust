use std::io::Read;

fn main() {
    let mut buf = String::new();
    std::io::stdin().read_to_string(&mut buf).unwrap();

    let answer = solve(&buf);

    println!("{}", answer);
}

fn solve(input: &str) -> String {
    let mut iterator = input.split_whitespace();

    let x: usize = iterator.next().unwrap().parse().unwrap();

    if x <= 599 {
        "8".to_string()
    } else if x <= 799 {
        "7".to_string()
    } else if x <= 999 {
        "6".to_string()
    } else if x <= 1199 {
        "5".to_string()
    } else if x <= 1399 {
        "4".to_string()
    } else if x <= 1599 {
        "3".to_string()
    } else if x <= 1799 {
        "2".to_string()
    } else {
        "1".to_string()
    }
}

#[test]
fn test() {
    let cases: Vec<(&str, &str)> = vec![
        (
            r"725",
            "7"
        ),
        (
            r"1600",
            "2"
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
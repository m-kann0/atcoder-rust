use std::io::Read;

fn main() {
    let mut buf = String::new();
    std::io::stdin().read_to_string(&mut buf).unwrap();

    let answer = solve(&buf);

    println!("{}", answer);
}

fn solve(input: &str) -> String {
    let mut iterator = input.split_whitespace();

    let mut a: usize = iterator.next().unwrap().parse().unwrap();
    let mut b: usize = iterator.next().unwrap().parse().unwrap();
    if a == 1 {
        a = 14;
    }
    if b == 1 {
        b = 14;
    }

    return if a == b {
        "Draw".to_string()
    } else if a > b {
        "Alice".to_string()
    } else {
        "Bob".to_string()
    };
}

#[test]
fn test() {
    let cases: Vec<(&str, &str)> = vec![
        (
            r"8 6",
            "Alice"
        ),
        (
            r"1 1",
            "Draw"
        ),
        (
            r"13 1",
            "Bob"
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
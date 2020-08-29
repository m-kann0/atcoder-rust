use std::io::Read;

fn main() {
    let mut buf = String::new();
    std::io::stdin().read_to_string(&mut buf).unwrap();

    let answer = solve(&buf);

    println!("{}", answer);
}

fn solve(input: &str) -> String {
    let mut iterator = input.split_whitespace();

    let d: f64 = iterator.next().unwrap().parse().unwrap();
    let t: f64 = iterator.next().unwrap().parse().unwrap();
    let s: f64 = iterator.next().unwrap().parse().unwrap();

    if d / s <= t {
        "Yes".to_string()
    } else {
        "No".to_string()
    }
}

#[test]
fn test() {
    let cases: Vec<(&str, &str)> = vec![
        (
            r"1000 15 80",
            "Yes"
        ),
        (
            r"2000 20 100",
            "Yes"
        ),
        (
            r"2000 20 99",
            "No"
        ),
        (
            r"10000 1 1",
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
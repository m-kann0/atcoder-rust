use std::io::Read;

fn main() {
    let mut buf = String::new();
    std::io::stdin().read_to_string(&mut buf).unwrap();

    let answer = solve(&buf);

    println!("{}", answer);
}

fn solve(input: &str) -> String {
    let mut iterator = input.split_whitespace();

    let n: i64 = iterator.next().unwrap().parse().unwrap();
    let m: i64 = iterator.next().unwrap().parse().unwrap();

    if n == 1 && m == 1 {
        return "1".to_string();
    }
    if n == 1 || m == 1 {
        return format!("{}", n * m - 2);
    }

    return format!("{}", (n - 2) * (m - 2));
}

#[test]
fn test() {
    let cases: Vec<(&str, &str)> = vec![
        (
            r"2 2",
            "0"
        ),
        (
            r"1 7",
            "5"
        ),
        (
            r"314 1592",
            "496080"
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
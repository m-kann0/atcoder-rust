use std::io::Read;

fn main() {
    let mut buf = String::new();
    std::io::stdin().read_to_string(&mut buf).unwrap();

    let answer = solve(&buf);

    println!("{}", answer);
}

fn solve(input: &str) -> String {
    let mut iterator = input.split_whitespace();

    let a: isize = iterator.next().unwrap().parse().unwrap();
    let b: isize = iterator.next().unwrap().parse().unwrap();
    let c: isize = iterator.next().unwrap().parse().unwrap();
    let d: isize = iterator.next().unwrap().parse().unwrap();

    if (a - c).abs() <= d || (a - b).abs() <= d && (b - c).abs() <= d {
        "Yes".to_string()
    } else {
        "No".to_string()
    }
}

#[test]
fn test() {
    let cases: Vec<(&str, &str)> = vec![
        (
            r"4 7 9 3",
            "Yes"
        ),
        (
            r"100 10 1 2",
            "No"
        ),
        (
            r"10 10 10 1",
            "Yes"
        ),
        (
            r"1 100 2 10",
            "Yes"
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
use std::io::Read;

fn main() {
    let mut buf = String::new();
    std::io::stdin().read_to_string(&mut buf).unwrap();

    let answer = solve(&buf);

    println!("{}", answer);
}

fn solve(input: &str) -> String {
    let mut iterator = input.split_whitespace();

    let x: isize = iterator.next().unwrap().parse().unwrap();
    let y: isize = iterator.next().unwrap().parse().unwrap();

    if (y - 2 * x) % 2 != 0 {
        return "No".to_string();
    }
    let t = (y - 2 * x) / 2;
    let s = x - t;
    if s >= 0 && t >= 0 {
        return "Yes".to_string();
    }
    return "No".to_string();
}

#[test]
fn test() {
    let cases: Vec<(&str, &str)> = vec![
        (
            r"3 8",
            "Yes"
        ),
        (
            r"2 100",
            "No"
        ),
        (
            r"1 2",
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
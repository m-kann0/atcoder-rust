use std::io::Read;

fn main() {
    let mut buf = String::new();
    std::io::stdin().read_to_string(&mut buf).unwrap();

    let answer = solve(&buf);

    println!("{}", answer);
}

fn solve(input: &str) -> String {
    let mut iterator = input.split_whitespace();

    let s = iterator.next().unwrap();
    let left: usize = s[0..2].parse().unwrap();
    let right: usize = s[2..4].parse().unwrap();

    if left >= 1 && left <= 12 && right >= 1 && right <= 12 {
        return "AMBIGUOUS".to_string();
    }
    if left >= 1 && left <= 12 {
        return "MMYY".to_string();
    }
    if right >= 1 && right <= 12 {
        return "YYMM".to_string();
    }
    return "NA".to_string();
}

#[test]
fn test() {
    let cases: Vec<(&str, &str)> = vec![
        (
            r"1905",
            "YYMM"
        ),
        (
            r"0112",
            "AMBIGUOUS"
        ),
        (
            r"1700",
            "NA"
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
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

    return if a == "H" && b == "H" || a == "D" && b == "D" {
        "H".to_string()
    } else {
        "D".to_string()
    };
}

#[test]
fn test() {
    let cases: Vec<(&str, &str)> = vec![
        (
            r"H H",
            "H"
        ),
        (
            r"D H",
            "D"
        ),
        (
            r"D D",
            "H"
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
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

    if s.starts_with("WBWBWWBWBWBW") {
        "Do".to_string()
    } else if s.starts_with("WBWWBWBWBWWB") {
        "Re".to_string()
    } else if s.starts_with("WWBWBWBWWBWB") {
        "Mi".to_string()
    } else if s.starts_with("WBWBWBWWBWBW") {
        "Fa".to_string()
    } else if s.starts_with("WBWBWWBWBWWB") {
        "So".to_string()
    } else if s.starts_with("WBWWBWBWWBWB") {
        "La".to_string()
    } else {
        "Si".to_string()
    }
}

#[test]
fn test() {
    let cases: Vec<(&str, &str)> = vec![
        (
            r"WBWBWWBWBWBWWBWBWWBW",
            "Do"
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
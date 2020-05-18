use std::io::Read;

fn main() {
    let mut buf = String::new();
    std::io::stdin().read_to_string(&mut buf).unwrap();

    let answer = solve(&buf);

    println!("{}", answer);
}

fn solve(input: &str) -> String {
    let mut iterator = input.split_whitespace();

    let n: usize = iterator.next().unwrap().parse().unwrap();
    return match n % 10 {
        3 => { "bon".to_string() }
        2 | 4 | 5 | 7 | 9 => { "hon".to_string() },
        _ => { "pon".to_string() }
    };
}

#[test]
fn test() {
    let cases: Vec<(&str, &str)> = vec![
        (
            r"16",
            "pon"
        ),
        (
            r"2",
            "hon"
        ),
        (
            r"183",
            "bon"
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
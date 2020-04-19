use std::io::Read;
use std::f64::consts::PI;

fn main() {
    let mut buf = String::new();
    std::io::stdin().read_to_string(&mut buf).unwrap();

    let answer = solve(&buf);

    println!("{}", answer);
}

fn solve(input: &str) -> String {
    let mut iterator = input.split_whitespace();

    let r: usize = iterator.next().unwrap().parse().unwrap();

    let ans: f64 = 2_f64 * (r as f64) * PI;

    return format!("{:.3}", ans);
}

#[test]
fn test() {
    let cases: Vec<(&str, &str)> = vec![
        (
            r"1",
            "6.28318530717958623200"
        ),
        (
            r"73",
            "458.67252742410977361942"
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
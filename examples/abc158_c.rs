use std::io::Read;
use std::cmp::max;

fn main() {
    let mut buf = String::new();
    std::io::stdin().read_to_string(&mut buf).unwrap();

    let answer = solve(&buf);

    println!("{}", answer);
}

fn solve(input: &str) -> String {
    let mut iterator = input.split_whitespace();

    let a: usize = iterator.next().unwrap().parse().unwrap();
    let b: usize = iterator.next().unwrap().parse().unwrap();

    let a1: usize = (a as f64 / 0.08).ceil() as usize;
    let a2: usize = ((a + 1) as f64 / 0.08).ceil() as usize;

    let b1: usize = (b as f64 / 0.10).ceil() as usize;
    let b2: usize = ((b + 1) as f64 / 0.10).ceil() as usize;

    if b1 >= a2 || a1 >= b2 {
        return "-1".to_string()
    }

    let ans = max(a1, b1);

    return ans.to_string();
}

#[test]
fn test() {
    let cases: Vec<(&str, &str)> = vec![
        (
            r"2 2",
            "25"
        ),
        (
            r"8 10",
            "100"
        ),
        (
            r"19 99",
            "-1"
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
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
    let k: usize = iterator.next().unwrap().parse().unwrap();

    let ans: f64 = ((k - 1) * (n - k) * 6 + (n - 1) * 3 + 1) as f64 / (n * n * n) as f64;
    return format!("{:.10}", ans);
}

#[test]
fn test() {
    let cases: Vec<(&str, &str)> = vec![
        (
            r"3 2",
            "0.48148148148148148148"
        ),
        (
            r"3 1",
            "0.25925925925925925926"
        ),
        (
            r"765 573",
            "0.00147697396984624371"
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
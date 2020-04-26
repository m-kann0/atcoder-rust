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

    let a: f64 = iterator.next().unwrap().parse().unwrap();
    let b: f64 = iterator.next().unwrap().parse().unwrap();
    let x: f64 = iterator.next().unwrap().parse().unwrap();

    let s = x / a;

    let ans = if s >= a * b / 2_f64 {
        let h = 2_f64 * (a * b - s) / a;
        h.atan2(a) * 360_f64 / (2_f64 * PI)
    } else {
        let w = 2_f64 * s / b;
        b.atan2(w) * 360_f64 / (2_f64 * PI)
    };

    return format!("{:.10}", ans);
}

#[test]
fn test() {
    let cases: Vec<(&str, &str)> = vec![
        (
            r"2 2 4",
            "45.0000000000"
        ),
        (
            r"12 21 10",
            "89.7834636934"
        ),
        (
            r"3 1 8",
            "4.2363947991"
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
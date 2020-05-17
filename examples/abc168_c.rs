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
    let h: usize = iterator.next().unwrap().parse().unwrap();
    let m: usize = iterator.next().unwrap().parse().unwrap();

    let minute: usize = h * 60 + m;
    let x_l: f64 = a * (1.0 / 2.0 * minute as f64 * PI / 180.0).sin();
    let y_l: f64 = - a * (1.0 / 2.0 * minute as f64 * PI / 180.0).cos();

    let x_s: f64 = b * (6.0 * m as f64 * PI / 180.0).sin();
    let y_s: f64 = - b * (6.0 * m as f64 * PI / 180.0).cos();

    let ans: f64 = ((x_l - x_s) * (x_l - x_s) + (y_l - y_s) * (y_l - y_s)).sqrt();
    return format!("{:.12}", ans);
}

#[test]
fn test() {
    let cases: Vec<(&str, &str)> = vec![
        (
            r"3 4 9 0",
            "5.00000000000000000000"
        ),
        (
            r"3 4 10 40",
            "4.56425719433005567605"
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
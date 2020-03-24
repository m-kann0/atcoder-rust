use std::io::Read;

fn main() {
    let mut buf = String::new();
    std::io::stdin().read_to_string(&mut buf).unwrap();

    let answer = solve(&buf);

    println!("{}", answer);
}

fn solve(input: &str) -> String {
    let mut iterator = input.split_whitespace();

    let n: isize = iterator.next().unwrap().parse().unwrap();
    let m: isize = iterator.next().unwrap().parse().unwrap();

    let n = n % 12;

    let dn = 360_f64 / 12_f64 * n as f64 + 360_f64 / (12_f64 * 60_f64) * m as f64;
    let dm = 360_f64 / 60_f64 * m as f64;

    let ans = (dn - dm).abs();
    let ans = if ans > 180_f64 {
        360_f64 - ans
    } else {
        ans
    };

    return ans.to_string();
}

#[test]
fn test() {
    let cases: Vec<(&str, &str)> = vec![
        (
            r"15 0",
            "90"
        ),
        (
            r"3 17",
            "3.5"
        ),
        (
            r"0 0",
            "0"
        ),
        (
            r"6 0",
            "180"
        ),
        (
            r"23 59",
            "5.5000"
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
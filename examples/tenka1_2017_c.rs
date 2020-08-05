use std::io::Read;

fn main() {
    let mut buf = String::new();
    std::io::stdin().read_to_string(&mut buf).unwrap();

    let answer = solve(&buf);

    println!("{}", answer);
}

fn solve(input: &str) -> String {
    let mut iterator = input.split_whitespace();

    let large_n: isize = iterator.next().unwrap().parse().unwrap();

    for h in 1..=3500 {
        for n in 1..=3500 {
            let denominator = 4 * h * n - large_n * n - large_n * h;
            if denominator <= 0 {
                continue;
            }
            let numerator = large_n * n * h;
            if numerator % denominator != 0 {
                continue;
            }
            let w = numerator / denominator;
            return format!("{} {} {}", h, n, w);
        }
    }

    return String::new();
}

#[test]
fn test() {
    let cases: Vec<(&str, &str)> = vec![
        (
            r"2",
            "1 2 2"
        ),
        (
            r"3485",
            "872 1012974 1539173474040"
        ),
        (
            r"4664",
            "3498 3498 3498"
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
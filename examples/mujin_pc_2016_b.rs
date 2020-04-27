use std::io::Read;
use std::cmp::max;
use std::f64::consts::PI;

fn main() {
    let mut buf = String::new();
    std::io::stdin().read_to_string(&mut buf).unwrap();

    let answer = solve(&buf);

    println!("{}", answer);
}

fn solve(input: &str) -> String {
    let mut iterator = input.split_whitespace();

    let a: u64 = iterator.next().unwrap().parse().unwrap();
    let b: u64 = iterator.next().unwrap().parse().unwrap();
    let c: u64 = iterator.next().unwrap().parse().unwrap();

    let max = max(a, max(b, c));
    let others = (a + b + c) - max;

    let sum: f64 = (a + b + c) as f64;
    let ans: f64 = if max < others {
        PI * sum * sum
    } else {
        let min: f64 = (max - others) as f64;
        PI * sum * sum - PI * min * min
    };
    return format!("{}", ans);
}

#[test]
fn test() {
    let cases: Vec<(&str, &str)> = vec![
        (
            r"1 1 1",
            r"28.2743338823"
        ),
        (
            r"3 1 1",
            r"75.3982236862"
        ),
        (
            r"16 2 27",
            r"6107.2561185786"
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
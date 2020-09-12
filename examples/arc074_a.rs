use std::io::Read;
use std::cmp::{min, max};

fn main() {
    let mut buf = String::new();
    std::io::stdin().read_to_string(&mut buf).unwrap();

    let answer = solve(&buf);

    println!("{}", answer);
}

fn solve(input: &str) -> String {
    let mut iterator = input.split_whitespace();

    let h: usize = iterator.next().unwrap().parse().unwrap();
    let w: usize = iterator.next().unwrap().parse().unwrap();

    if h % 3 == 0 || w % 3 == 0 {
        return "0".to_string();
    }

    let result = min(calc(h, w), calc(w, h));
    result.to_string()
}

fn calc(h: usize, w: usize) -> usize {
    let mut result = std::usize::MAX;
    for h1 in 1..h {
        let s1 = h1 * w;

        let s2 = (h - h1) * (w / 2);
        let s3 = h * w - s1 - s2;
        let smax = max(s1, max(s2, s3));
        let smin = min(s1, min(s2, s3));
        result = min(result, smax - smin);

        let s2 = ((h - h1) / 2) * w;
        let s3 = h * w - s1 - s2;
        let smax = max(s1, max(s2, s3));
        let smin = min(s1, min(s2, s3));
        result = min(result, smax - smin);
    }
    result
}

#[test]
fn test() {
    let cases: Vec<(&str, &str)> = vec![
        (
            r"3 5",
            "0"
        ),
        (
            r"4 5",
            "2"
        ),
        (
            r"5 5",
            "4"
        ),
        (
            r"100000 2",
            "1"
        ),
        (
            r"100000 100000",
            "50000"
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
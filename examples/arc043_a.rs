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

    let n: isize = iterator.next().unwrap().parse().unwrap();
    let a: isize = iterator.next().unwrap().parse().unwrap();
    let b: isize = iterator.next().unwrap().parse().unwrap();

    // let s: Vec<isize> = (0..n)
    //     .map(|_| iterator.next().unwrap().parse().unwrap())
    //     .collect();
    //
    // let s_max = s.iter().max().unwrap();
    // let s_min = s.iter().min().unwrap();
    // let s_total: isize = s.iter().fold(0, |s, x| s + *x);

    let mut s_min: isize = 1_000_000_001;
    let mut s_max: isize = -1;
    let mut s_total: isize = 0;
    for _ in 0..n {
        let si = iterator.next().unwrap().parse().unwrap();
        s_min = min(s_min, si);
        s_max = max(s_max, si);
        s_total += si;
    }

    if s_max == s_min {
        return "-1".to_string();
    }


    let p: f64 = b as f64 / (s_max - s_min) as f64;
    let q: f64 = a as f64 - (p * s_total as f64) / n as f64;
    return format!("{} {}", p, q);
}

#[test]
fn test() {
    let cases: Vec<(&str, &str)> = vec![
        (
            r"5 2 4
2
4
6
8
10",
            "0.5 -1"
        ),
        (
            r"13 29 31
3
1
4
1
5
9
2
6
5
3
5
8
9",
            "3.875 10.8173076"
        ),
        (
            r"5 1 2
34
34
34
34
34",
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
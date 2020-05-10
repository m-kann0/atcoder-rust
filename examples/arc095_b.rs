use std::io::Read;
use std::cmp::{max, min};

fn main() {
    let mut buf = String::new();
    std::io::stdin().read_to_string(&mut buf).unwrap();

    let answer = solve(&buf);

    println!("{}", answer);
}

fn solve(input: &str) -> String {
    let mut iterator = input.split_whitespace();

    let n: usize = iterator.next().unwrap().parse().unwrap();
    let mut a: Vec<usize> = Vec::new();
    let mut a_max: usize = 0;
    for _ in 0..n {
        let ai: usize = iterator.next().unwrap().parse().unwrap();
        a.push(ai);
        a_max = max(a_max, ai);
    }

    let mut b_max = 0;
    let mut b = 0;
    for i in 0..n {
        let bi: usize = a[i];
        if bi == a_max {
            continue;
        }
        if b_max < min(bi, a_max - bi) {
            b_max = min(bi, a_max - bi);
            b = bi;
        }
    }

    return format!("{} {}", a_max, b);
}

#[test]
fn test() {
    let cases: Vec<(&str, &str)> = vec![
        (
            r"5
6 9 4 2 11",
            "11 6"
        ),
        (
            r"2
100 0",
            "100 0"
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
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

    let c: usize = iterator.next().unwrap().parse().unwrap();

    let mut e1_max: usize = 0;
    let mut e2_max: usize = 0;
    let mut e3_max: usize = 0;
    for _ in 0..c {
        let n: usize = iterator.next().unwrap().parse().unwrap();
        let m: usize = iterator.next().unwrap().parse().unwrap();
        let l: usize = iterator.next().unwrap().parse().unwrap();
        let e1: usize = min(n, min(m, l));
        let e3: usize = max(n, max(m, l));
        let e2: usize = n + m + l - e1 - e3;
        e1_max = max(e1_max, e1);
        e2_max = max(e2_max, e2);
        e3_max = max(e3_max, e3);
    }

    let ans = e1_max * e2_max * e3_max;
    return ans.to_string();
}

#[test]
fn test() {
    let cases: Vec<(&str, &str)> = vec![
        (
            r"2
10 20 30
20 20 20",
            r"12000"
        ),
        (
            r"3
10 20 30
20 20 20
30 20 10",
            r"12000"
        ),
        (
            r"4
10 20 30
20 20 20
30 20 10
10 40 10",
            r"16000"
        ),
        (
            r"2
10 10 10
11 1 1",
            r"1100"
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
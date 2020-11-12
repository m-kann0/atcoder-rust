#![allow(non_snake_case)]

use std::io::Read;
use std::cmp::min;

fn main() {
    let mut buf = String::new();
    std::io::stdin().read_to_string(&mut buf).unwrap();

    let answer = solve(&buf);

    println!("{}", answer);
}

fn solve(input: &str) -> String {
    let mut iterator = input.split_whitespace();

    let A: usize = iterator.next().unwrap().parse().unwrap();
    let B: usize = iterator.next().unwrap().parse().unwrap();
    let M: usize = iterator.next().unwrap().parse().unwrap();
    let a: Vec<usize> = (0..A).map(|_| iterator.next().unwrap().parse().unwrap()).collect();
    let b: Vec<usize> = (0..B).map(|_| iterator.next().unwrap().parse().unwrap()).collect();
    let mut ans = a.iter().min().unwrap() + b.iter().min().unwrap();
    for _ in 0..M {
        let x: usize = iterator.next().unwrap().parse().unwrap();
        let y: usize = iterator.next().unwrap().parse().unwrap();
        let c: usize = iterator.next().unwrap().parse().unwrap();
        let now = a[x - 1] + b[y - 1] - c;
        ans = min(ans, now);
    }
    ans.to_string()
}

#[test]
fn test() {
    let cases: Vec<(&str, &str)> = vec![
        (
            r"2 3 1
3 3
3 3 3
1 2 1",
            "5"
        ),
        (
            r"1 1 2
10
10
1 1 5
1 1 10",
            "10"
        ),
        (
            r"2 2 1
3 5
3 5
2 2 2",
            "6"
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
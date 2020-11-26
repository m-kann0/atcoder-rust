#![allow(non_snake_case)]

use std::io::Read;
use std::collections::BinaryHeap;
use std::cmp::Reverse;

fn main() {
    let mut buf = String::new();
    std::io::stdin().read_to_string(&mut buf).unwrap();

    let answer = solve(&buf);

    println!("{}", answer);
}

fn solve(input: &str) -> String {
    let mut iterator = input.split_whitespace();

    let n: usize = iterator.next().unwrap().parse().unwrap();
    let k: usize = iterator.next().unwrap().parse().unwrap();
    let mut pq = BinaryHeap::new();
    for _ in 0..n {
        let ai: usize = iterator.next().unwrap().parse().unwrap();
        let bi: usize = iterator.next().unwrap().parse().unwrap();
        pq.push(Reverse((ai, bi)));
    }

    let mut ans = 0;
    for _ in 0..k {
        let machine = pq.pop().unwrap().0;
        ans += machine.0;
        pq.push(Reverse((machine.0 + machine.1, machine.1)));
    }
    ans.to_string()
}

#[test]
fn test() {
    let cases: Vec<(&str, &str)> = vec![
        (
            r"3 3
1 3
2 0
3 4",
            "5"
        ),
        (
            r"10 100000
22 59
26 60
72 72
47 3
97 16
75 41
82 77
17 97
32 32
28 7",
            "7521307799"
        ),
        (
            r"1 100000
1000000000 1000000000",
            "5000050000000000000"
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
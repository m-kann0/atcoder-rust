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

    let d: isize = iterator.next().unwrap().parse().unwrap();
    let n: usize = iterator.next().unwrap().parse().unwrap();
    let m: usize = iterator.next().unwrap().parse().unwrap();
    let mut store = vec![0];
    for _ in 0..(n - 1) {
        let di: isize = iterator.next().unwrap().parse().unwrap();
        store.push(di);
    }
    let q: Vec<isize> = (0..m).map(|_| iterator.next().unwrap().parse().unwrap()).collect();

    store.sort();

    let mut ans: isize = 0;
    for i in 0..m {
        let mut ok: isize = 0;
        let mut ng: isize = n as isize;
        while (ok - ng).abs() > 1 {
            let mid = (ok + ng) / 2;
            if store[mid as usize] <= q[i] {
                ok = mid;
            } else {
                ng = mid;
            }
        }
        let ok = ok as usize;
        if ok + 1 <= n - 1 {
            ans += min((store[ok] - q[i]).abs(), (store[ok + 1] - q[i]).abs());
        } else {
            ans += min((store[ok] - q[i]).abs(), (d - q[i]).abs());
        }
    }
    ans.to_string()
}

#[test]
fn test() {
    let cases: Vec<(&str, &str)> = vec![
        (
            r"8
3
2
3
1
4
6
",
            "3"
        ),
        (
            r"20
4
4
12
8
16
7
7
11
8
",
            "3"
        ),
    ];

    let mut all_ok = true;
    for (i, case) in cases.iter().enumerate() {
        print!("case {} : ", i);

        let expected = case.1;
        let actual = solve(case.0);

        if expected.trim() == actual.trim() {
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
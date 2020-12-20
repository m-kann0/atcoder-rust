#![allow(non_snake_case)]

use std::io::Read;

fn main() {
    let mut buf = String::new();
    std::io::stdin().read_to_string(&mut buf).unwrap();

    let answer = solve(&buf);

    println!("{}", answer);
}

fn solve(input: &str) -> String {
    let mut iterator = input.split_whitespace();

    let n: usize = iterator.next().unwrap().parse().unwrap();
    let a: Vec<isize> = (0..n).map(|_| iterator.next().unwrap().parse().unwrap()).collect();

    let mut b = vec![-1; n];
    for i in 0..n {
        let mut ok = n as isize - 1;
        let mut ng = -1 as isize;
        while (ok - ng).abs() > 1 {
            let mid = (ok + ng) / 2;
            if b[mid as usize] < a[i] {
                ok = mid;
            } else {
                ng = mid;
            }
        }
        b[ok as usize] = a[i];
    }

    let ans: usize = b.iter().filter(|bi| **bi >= 0).count();
    ans.to_string()
}

#[test]
fn test() {
    let cases: Vec<(&str, &str)> = vec![
        (
            r"5
2
1
4
5
3",
            "2"
        ),
        (
            r"4
0
0
0
0",
            "4"
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
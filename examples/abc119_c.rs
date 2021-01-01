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

    let n: usize = iterator.next().unwrap().parse().unwrap();
    let a: usize = iterator.next().unwrap().parse().unwrap();
    let b: usize = iterator.next().unwrap().parse().unwrap();
    let c: usize = iterator.next().unwrap().parse().unwrap();
    let l: Vec<usize> = (0..n).map(|_| iterator.next().unwrap().parse().unwrap()).collect();

    let abc = vec![a, b, c];

    let mut ans: usize = std::usize::MAX;
    for bit in 0..(1 << (2 * n)) {
        let mut ls = vec![vec![]; 4];
        for i in 0..n {
            let t = ((bit >> (i * 2 + 1)) & 1) * 2 + ((bit >> (i * 2)) & 1);
            ls[t].push(l[i]);
        }
        if ls[0].is_empty() || ls[1].is_empty() || ls[2].is_empty() {
            continue;
        }
        let mut now = 0;
        for i in 0..3 {
            now += (ls[i].len() - 1) * 10;
            now += (abc[i] as isize - ls[i].iter().sum::<usize>() as isize).abs() as usize;
        }
        ans = min(ans, now);
    }
    ans.to_string()
}

#[test]
fn test() {
    let cases: Vec<(&str, &str)> = vec![
        (
            r"5 100 90 80
98
40
30
21
80",
            "23"
        ),
        (
            r"8 100 90 80
100
100
90
90
90
80
80
80",
            "0"
        ),
        (
            r"8 1000 800 100
300
333
400
444
500
555
600
666",
            "243"
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
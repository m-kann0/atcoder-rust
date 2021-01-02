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
    let mut v = Vec::new();
    let mut aoki = 0;
    let mut taka = 0;
    for _ in 0..n {
        let ai: usize = iterator.next().unwrap().parse().unwrap();
        let bi: usize = iterator.next().unwrap().parse().unwrap();
        let key = ai * 2 + bi;
        v.push((key, ai, bi));
        aoki += ai;
    }

    v.sort();
    v.reverse();

    let mut count = 0;
    for i in 0..n {
        if taka > aoki {
            break;
        }
        aoki -= v[i].1;
        taka += v[i].1 + v[i].2;
        count += 1;
    }
    count.to_string()
}

#[test]
fn test() {
    let cases: Vec<(&str, &str)> = vec![
        (
            r"4
2 1
2 2
5 1
1 3",
            "1"
        ),
        (
            r"5
2 1
2 1
2 1
2 1
2 1",
            "3"
        ),
        (
            r"1
273 691",
            "1"
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
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
    let m: usize = iterator.next().unwrap().parse().unwrap();
    let mut a: Vec<usize> = (0..m).map(|_| iterator.next().unwrap().parse().unwrap()).collect();

    a.push(0);
    a.push(n + 1);
    a.sort();
    let mut b = Vec::new();
    for i in 1..a.len() {
        b.push(a[i] - a[i - 1] - 1);
    }
    let c: Vec<usize> = b.into_iter().filter(|x| *x != 0).map(|x| x).collect();
    if c.is_empty() {
        return "0".to_string();
    }
    let k: usize = *c.iter().min().unwrap();
    // eprintln!("k = {:?}", k);
    let mut ans = 0;
    for i in 0..c.len() {
        // eprintln!("c[i] = {:?}", c[i]);
        ans += (c[i] + k - 1) / k;
    }
    ans.to_string()
}

#[test]
fn test() {
    let cases: Vec<(&str, &str)> = vec![
        (
            r"5 2
1 3",
            "3"
        ),
        (
            r"13 3
13 3 9",
            "6"
        ),
        (
            r"13 2
3 9",
            "6"
        ),
        (
            r"5 5
5 2 1 4 3",
            "0"
        ),
        (
            r"1 0",
            "1"
        ),
        (
            r"5 0",
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
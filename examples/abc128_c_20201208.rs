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
    let mut s = vec![vec![]; m];
    for i in 0..m {
        let k: usize = iterator.next().unwrap().parse().unwrap();
        for _ in 0..k {
            let sij: usize = iterator.next().unwrap().parse().unwrap();
            s[i].push(sij - 1);
        }
    }
    let p: Vec<usize> = (0..m).map(|_| iterator.next().unwrap().parse().unwrap()).collect();

    let mut ans: usize = 0;
    for bit in 0..(1<<n) {
        let mut is_ok = true;
        for i in 0..m {
            let mut count = 0;
            for &j in &s[i] {
                if bit>>j & 1 > 0 {
                    count += 1;
                }
            }
            if count % 2 != p[i] {
                is_ok = false;
                break;
            }
        }
        if is_ok {
            ans += 1;
        }
    }
    ans.to_string()
}

#[test]
fn test() {
    let cases: Vec<(&str, &str)> = vec![
        (
            r"2 2
2 1 2
1 2
0 1",
            "1"
        ),
        (
            r"2 3
2 1 2
1 1
1 2
0 0 1",
            "0"
        ),
        (
            r"5 2
3 1 2 5
2 2 3
1 0",
            "8"
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
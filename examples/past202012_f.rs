#![allow(non_snake_case)]

use std::io::Read;
use std::collections::HashSet;
use std::cmp::max;

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
    let mut a = Vec::new();
    let mut b = Vec::new();
    let mut c = Vec::new();
    for _ in 0..m {
        let ai: usize = iterator.next().unwrap().parse().unwrap();
        let bi: usize = iterator.next().unwrap().parse().unwrap();
        let ci: usize = iterator.next().unwrap().parse().unwrap();
        a.push(ai - 1);
        b.push(bi - 1);
        c.push(ci - 1);
    }

    let mut ans: usize = 0;
    for bit in 0..(1<<n) {
        let mut is_ok = true;
        let mut set = HashSet::new();
        for i in 0..m {
            let mut count = 0;
            if bit >> a[i] & 1 > 0 {
                count += 1;
            }
            if bit >> b[i] & 1 > 0 {
                count += 1;
            }
            if bit >> c[i] & 1 > 0 {
                count += 1;
            }
            if count == 3 {
                is_ok = false;
                break;
            }
            if count == 2 {
                if bit >> a[i] & 1 == 0 {
                    set.insert(a[i]);
                }
                if bit >> b[i] & 1 == 0 {
                    set.insert(b[i]);
                }
                if bit >> c[i] & 1 == 0 {
                    set.insert(c[i]);
                }
            }
        }
        if is_ok {
            ans = max(ans, set.len());
        }
    }
    ans.to_string()
}

#[test]
fn test() {
    let cases: Vec<(&str, &str)> = vec![
        (
            r"4 2
1 2 3
1 2 4",
            "2"
        ),
        (
            r"6 7
1 2 5
2 3 5
2 4 5
1 2 3
4 5 6
2 5 6
1 3 5",
            "4"
        ),
        (
            r"5 1
1 2 3",
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
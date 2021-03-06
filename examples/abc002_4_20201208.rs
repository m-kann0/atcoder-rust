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
    let mut p = HashSet::new();
    for _ in 0..m {
        let x: usize = iterator.next().unwrap().parse().unwrap();
        let y: usize = iterator.next().unwrap().parse().unwrap();
        p.insert((x - 1, y - 1));
    }

    let mut ans = 1;
    for bit in 0..(1<<n) {
        let mut is_ok = true;
        for i in 0..n {
            if bit>>i & 1 == 0 {
                continue;
            }
            for j in (i + 1)..n {
                if bit>>j & 1 == 0 {
                    continue;
                }
                if !p.contains(&(i, j)) {
                    is_ok = false;
                }
            }
        }
        if is_ok {
            ans = max(ans, (bit as usize).count_ones());
        }
    }
    ans.to_string()
}

#[test]
fn test() {
    let cases: Vec<(&str, &str)> = vec![
        (
            r"5 3
1 2
2 3
1 3
",
            "3"
        ),
        (
            r"5 3
1 2
2 3
3 4
",
            "2"
        ),
        (
            r"7 9
1 2
1 3
2 3
4 5
4 6
4 7
5 6
5 7
6 7
",
            "4"
        ),
        (
            r"12 0
",
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
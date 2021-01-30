#![allow(non_snake_case)]

use std::io::Read;
use std::collections::HashMap;
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
    let mut counts: HashMap<(usize, usize), usize> = HashMap::new();
    for _ in 0..m {
        let a: usize = iterator.next().unwrap().parse().unwrap();
        let b: usize = iterator.next().unwrap().parse().unwrap();
        *counts.entry((a, b)).or_insert(0) += 1;
    }
    let k: usize = iterator.next().unwrap().parse().unwrap();
    let mut c = Vec::new();
    let mut d = Vec::new();
    for _ in 0..k {
        let ci: usize = iterator.next().unwrap().parse().unwrap();
        let di: usize = iterator.next().unwrap().parse().unwrap();
        c.push(ci);
        d.push(di);
    }

    let mut ans = 0;
    for bit in 0..(1 << k) {
        let mut placed = vec![false; n + 1];
        for i in 0..k {
            if bit & (1 << i) > 0 {
                placed[c[i]] = true;
            } else {
                placed[d[i]] = true;
            }
        }
        let mut now = 0;
        for (&k, &v) in counts.iter() {
            if placed[k.0] && placed[k.1] {
                now += v;
            }
        }
        ans = max(ans, now);
    }
    ans.to_string()
}

#[test]
fn test() {
    let cases: Vec<(&str, &str)> = vec![
        (
            r"4 4
1 2
1 3
2 4
3 4
3
1 2
1 3
2 3",
            "2"
        ),
        (
            r"4 4
1 2
1 3
2 4
3 4
4
3 4
1 2
2 4
2 4",
            "4"
        ),
        (
            r"6 12
2 3
4 6
1 2
4 5
2 6
1 5
4 5
1 3
1 2
2 6
2 3
2 5
5
3 5
1 4
2 6
4 6
5 6",
            "9"
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
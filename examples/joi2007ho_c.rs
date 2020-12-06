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
    let mut v = Vec::new();
    let mut s = HashSet::new();
    for _ in 0..n {
        let x: isize = iterator.next().unwrap().parse().unwrap();
        let y: isize = iterator.next().unwrap().parse().unwrap();
        v.push((x, y));
        s.insert((x, y));
    }

    let mut ans = 0;
    for i in 0..n {
        for j in (i + 1)..n {
            let x1 = v[i].0;
            let y1 = v[i].1;
            let x2 = v[j].0;
            let y2 = v[j].1;

            let dx = x2 - x1;
            let dy = y2 - y1;

            let x3 = x1 - dy;
            let x4 = x2 - dy;
            let y3 = y1 + dx;
            let y4 = y2 + dx;

            if s.contains(&(x3, y3)) && s.contains(&(x4, y4)) {
                ans = max(ans, dx * dx + dy * dy);
            }

            let x3 = x1 + dy;
            let x4 = x2 + dy;
            let y3 = y1 - dx;
            let y4 = y2 - dx;

            if s.contains(&(x3, y3)) && s.contains(&(x4, y4)) {
                ans = max(ans, dx * dx + dy * dy);
            }
        }
    }
    ans.to_string()
}

#[test]
fn test() {
    let cases: Vec<(&str, &str)> = vec![
        (
            r"10
9 4
4 3
1 1
4 2
2 4
5 8
4 0
5 3
0 5
5 2",
            "10"
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
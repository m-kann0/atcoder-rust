#![allow(non_snake_case)]

use std::io::Read;
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
    let mut a = vec![vec![0_isize; 3]; n];
    for i in 0..n {
        for j in 0..3 {
            a[i][j] = iterator.next().unwrap().parse().unwrap();
        }
    }

    let mut ans: isize = 0;
    for bit in 0..(1 << 3) {
        let mut b = Vec::new();
        for i in 0..n {
            let mut key = 0;
            for j in 0..3 {
                let sign = if bit >> j & 1 > 0 {
                    1
                } else {
                    -1
                };
                key += sign * a[i][j];
            }
            b.push((key, a[i][0], a[i][1], a[i][2]));
        }
        b.sort();
        b.reverse();
        let mut x: isize = 0;
        let mut y: isize = 0;
        let mut z: isize = 0;
        for i in 0..m {
            x += b[i].1;
            y += b[i].2;
            z += b[i].3;
        }
        let now = x.abs() + y.abs() + z.abs();
        ans = max(ans, now);
    }
    ans.to_string()
}

#[test]
fn test() {
    let cases: Vec<(&str, &str)> = vec![
        (
            r"5 3
3 1 4
1 5 9
2 6 5
3 5 8
9 7 9",
            "56"
        ),
        (
            r"5 3
1 -2 3
-4 5 -6
7 -8 -9
-10 11 -12
13 -14 15",
            "54"
        ),
        (
            r"10 5
10 -80 21
23 8 38
-94 28 11
-26 -2 18
-69 72 79
-26 -86 -54
-72 -50 59
21 65 -32
40 -94 87
-62 18 82",
            "638"
        ),
        (
            r"3 2
2000000000 -9000000000 4000000000
7000000000 -5000000000 3000000000
6000000000 -1000000000 8000000000",
            "30000000000"
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
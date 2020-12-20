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

    let q: usize = iterator.next().unwrap().parse().unwrap();

    let mut result = String::new();
    for _ in 0..q {
        let n: isize = iterator.next().unwrap().parse().unwrap();
        let s: isize = iterator.next().unwrap().parse().unwrap();
        let k: isize = iterator.next().unwrap().parse().unwrap();

        let (g, x, y) = extgcd(k, n);
        if s % g != 0 {
            result.push_str(&format!("{}\n", -1));
            continue;
        }
        let n = n / g;
        let s = s / g;
        let k = k / g;
        let ans = ((x * -s) % n + n) % n;
        result.push_str(&format!("{}\n", ans));
    }
    result
}

// ax + by = g となるような (g, x, y) を返す
fn extgcd(a: isize, b: isize) -> (isize, isize, isize) {
    if b == 0 {
        return (a, 1, 0);
    }
    let (g, x, y) = extgcd(b, a % b);
    (g, y, x - a / b * y)
}

#[test]
fn test() {
    let cases: Vec<(&str, &str)> = vec![
        (
            r"4
10 4 3
1000 11 2
998244353 897581057 595591169
10000 6 14",
            "2
-1
249561088
3571"
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
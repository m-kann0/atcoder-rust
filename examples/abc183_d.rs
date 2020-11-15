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
    let w: isize = iterator.next().unwrap().parse().unwrap();
    let mut s: Vec<isize> = vec![0; 200_005];
    for _ in 0..n {
        let si: usize = iterator.next().unwrap().parse().unwrap();
        let ti: usize = iterator.next().unwrap().parse().unwrap();
        let pi: isize = iterator.next().unwrap().parse().unwrap();
        s[si] += pi;
        s[ti] -= pi;
    }

    for i in 1..200_005 {
        s[i] += s[i - 1];
    }
    let mut ans = 0;
    for i in 0..200_005 {
        ans = max(ans, s[i]);
    }
    if ans > w {
        return "No".to_string();
    }
    "Yes".to_string()
}

#[test]
fn test() {
    let cases: Vec<(&str, &str)> = vec![
        (
            r"4 10
1 3 5
2 4 4
3 10 6
2 4 1",
            "No"
        ),
        (
            r"4 10
1 3 5
2 4 4
3 10 6
2 3 1",
            "Yes"
        ),
        (
            r"6 1000000000
0 200000 999999999
2 20 1
20 200 1
200 2000 1
2000 20000 1
20000 200000 1",
            "Yes"
        ),
    ];

    let mut all_ok = true;
    for (i, case) in cases.iter().enumerate() {
        print!("case {} : ", i);

        let expected = case.1;
        let actual = solve(case.0);

        if expected == actual {
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

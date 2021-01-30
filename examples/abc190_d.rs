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

    let s: isize = iterator.next().unwrap().parse().unwrap();

    let mut ans = 0;
    let mut n = 1;
    while n * n <= 2 * s {
        if (2 * s - n * n + n) % (2 * n) == 0 {
            ans += 2;
        }
        n += 1;
    }
    // let mut l = 1;
    // while l * l <= 2 * n {
    //     let mut ok = l;
    //     let mut ng = n;
    //     while (ok - ng).abs() > 1 {
    //         let mid = (ok + ng) / 2;
    //         let result = mid * mid + mid + l - l * l;
    //         if result == 2 * n {
    //             ans += 2;
    //             break;
    //         } else if result < 2 * n {
    //             ok = mid;
    //         } else {
    //             ng = mid;
    //         }
    //     }
    //     l += 1;
    // }
    ans.to_string()
}

#[test]
fn test() {
    let cases: Vec<(&str, &str)> = vec![
        (
            r"12",
            "4"
        ),
        (
            r"1",
            "2"
        ),
        (
            r"963761198400",
            "1920"
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
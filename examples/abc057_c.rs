use std::io::Read;
use std::cmp::{max, min};

fn main() {
    let mut buf = String::new();
    std::io::stdin().read_to_string(&mut buf).unwrap();

    let answer = solve(&buf);

    println!("{}", answer);
}

fn solve(input: &str) -> String {
    let mut iterator = input.split_whitespace();

    let n: usize = iterator.next().unwrap().parse().unwrap();

    let mut ans: usize = 100;
    let mut a: usize = 1;
    while a * a <= n {
        if n % a != 0 {
            a += 1;
            continue;
        }
        let b = n / a;
        ans = min(ans, f(a, b));
        a += 1;
    }
    return ans.to_string();
}

fn f(a: usize, b: usize) -> usize {
    let a_len = format!("{}", a).len();
    let b_len = format!("{}", b).len();
    return max(a_len, b_len);
}

#[test]
fn test() {
    let cases: Vec<(&str, &str)> = vec![
        (
            r"10000",
            "3"
        ),
        (
            r"1000003",
            "7"
        ),
        (
            r"9876543210",
            "6"
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
use std::io::Read;
use std::cmp::{min, max};

fn main() {
    let mut buf = String::new();
    std::io::stdin().read_to_string(&mut buf).unwrap();

    let answer = solve(&buf);

    println!("{}", answer);
}

fn solve(input: &str) -> String {
    let mut iterator = input.split_whitespace();

    let n: usize = iterator.next().unwrap().parse().unwrap();
    let t: Vec<usize> = (0..n).map(|_| iterator.next().unwrap().parse().unwrap()).collect();

    let mut ans: usize = std::usize::MAX;
    for bit in 0..(1 << n) {
        let mut t1: usize = 0;
        let mut t2: usize = 0;

        for i in 0..n {
            if bit & 1 << i > 0 {
                t1 += t[i];
            } else {
                t2 += t[i];
            }
        }

        ans = min(ans, max(t1, t2));
    }
    ans.to_string()
}

#[test]
fn test() {
    let cases: Vec<(&str, &str)> = vec![
        (
            r"4
4
6
7
10",
            "14"
        ),
        (
            r"3
1
2
4",
            "4"
        ),
        (
            r"1
29",
            "29"
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
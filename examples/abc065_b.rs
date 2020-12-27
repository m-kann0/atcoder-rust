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

    let n: usize = iterator.next().unwrap().parse().unwrap();
    let a: Vec<usize> = (0..n).map(|_|
        iterator.next().unwrap().parse::<usize>().unwrap() - 1
    ).collect();

    let mut current: usize = 0;
    let mut ans: usize = 0;
    for _ in 0..(n + 5) {
        if current == 1 {
            return ans.to_string();
        }
        current = a[current];
        ans += 1;
    }
    "-1".to_string()
}

#[test]
fn test() {
    let cases: Vec<(&str, &str)> = vec![
        (
            r"3
3
1
2",
            "2"
        ),
        (
            r"4
3
4
1
2",
            "-1"
        ),
        (
            r"5
3
3
4
2
4",
            "3"
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
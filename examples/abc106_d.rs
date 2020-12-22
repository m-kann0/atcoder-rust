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
    let m: usize = iterator.next().unwrap().parse().unwrap();
    let q: usize = iterator.next().unwrap().parse().unwrap();

    let mut sum = vec![vec![0_usize; n + 1]; n + 1];
    for _ in 0..m {
        let l: usize = iterator.next().unwrap().parse().unwrap();
        let r: usize = iterator.next().unwrap().parse().unwrap();
        sum[l][r] += 1;
    }

    for l in 0..=n {
        for r in 1..=n {
            sum[l][r] += sum[l][r - 1];
        }
    }

    let mut result = String::new();
    for _ in 0..q {
        let p: usize = iterator.next().unwrap().parse().unwrap();
        let q: usize = iterator.next().unwrap().parse().unwrap();

        let mut now: usize = 0;
        for i in p..=q {
            now += sum[i][q];
        }
        result.push_str(&format!("{}\n", now));
    }
    result
}

#[test]
fn test() {
    let cases: Vec<(&str, &str)> = vec![
        (
            r"2 3 1
1 1
1 2
2 2
1 2",
            "3"
        ),
        (
            r"10 3 2
1 5
2 8
7 10
1 7
3 10",
            "1
1"
        ),
        (
            r"10 10 10
1 6
2 9
4 5
4 7
4 7
5 8
6 6
6 7
7 9
10 10
1 8
1 9
1 10
2 8
2 9
2 10
3 8
3 9
3 10
1 10",
            "7
9
10
6
8
9
6
7
8
10"
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
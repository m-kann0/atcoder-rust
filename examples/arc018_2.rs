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
    let mut x = Vec::new();
    let mut y = Vec::new();
    for _ in 0..n {
        let xi: isize = iterator.next().unwrap().parse().unwrap();
        let yi: isize = iterator.next().unwrap().parse().unwrap();
        x.push(xi);
        y.push(yi);
    }

    let mut ans: usize = 0;
    for i in 0..n {
        for j in 0..n {
            for k in 0..n {
                if i < j && j < k {
                    let a = ((x[i] - x[k]) * (y[j] - y[k]) - (x[j] - x[k]) * (y[i] - y[k])).abs();
                    if a > 0 && a % 2 == 0 {
                        ans += 1;
                    }
                }
            }
        }
    }
    ans.to_string()
}

#[test]
fn test() {
    let cases: Vec<(&str, &str)> = vec![
        (
            r"3
1 1
1 2
3 1
",
            "1"
        ),
        (
            r"3
1 1
1 2
2 1
",
            "0"
        ),
        (
            r"3
1 1
2 2
3 3
",
            "0"
        ),
        (
            r"8
3 1
4 1
5 9
2 6
5 3
5 8
9 7
9 3
",
            "38"
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
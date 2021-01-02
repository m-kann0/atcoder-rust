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
            if i >= j {
                continue;
            }
            let x_diff = (x[i] - x[j]).abs();
            let y_diff = (y[i] - y[j]).abs();
            if x_diff >= y_diff {
                ans += 1;
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
0 0
1 2
2 1",
            "2"
        ),
        (
            r"1
-691 273",
            "0"
        ),
        (
            r"10
-31 -35
8 -36
22 64
5 73
-14 8
18 -58
-41 -85
1 -88
-21 -85
-11 82",
            "11"
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
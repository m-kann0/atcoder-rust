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
    let a: Vec<usize> = (0..n).map(|_| iterator.next().unwrap().parse().unwrap()).collect();

    let mut exist = vec![false; 8];
    let mut over3200 = 0;
    for i in 0..n {
        if a[i] / 400 < 8 {
            exist[a[i] / 400] = true;
        } else {
            over3200 += 1;
        }
    }

    let mut mn = 0;
    for i in 0..8 {
        if exist[i] {
            mn += 1;
        }
    }

    let mx = mn + over3200;
    if mn == 0 {
        mn = 1;
    }

    format!("{} {}", mn, mx)
}

#[test]
fn test() {
    let cases: Vec<(&str, &str)> = vec![
        (
            r"4
2100 2500 2700 2700",
            "2 2"
        ),
        (
            r"5
1100 1900 2800 3200 3200",
            "3 5"
        ),
        (
            r"20
800 810 820 830 840 850 860 870 880 890 900 910 920 930 940 950 960 970 980 990",
            "1 1"
        ),
        (
            r"4
3200 3200 3200 3200",
            "1 4"
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
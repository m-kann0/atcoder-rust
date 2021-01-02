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

    let mut x = 1;
    let mut add = 2;
    while x < n {
        x += add;
        add += 1;
    }

    if x != n {
        return "No".to_string();
    }

    let mut result = vec![vec![]; add];
    let mut d = 1_usize;
    for i in 0..add {
        for j in i + 1..add {
            result[i].push(d);
            result[j].push(d);
            d += 1;
        }
    }

    let mut output = String::new();
    output.push_str(&format!("{}\n", "Yes"));
    output.push_str(&format!("{}\n", result.len()));
    for v in result {
        output.push_str(&format!("{}", v.len()));
        for a in v {
            output.push_str(&format!(" {}", a));
        }
        output.push('\n');
    }
    output
}

#[test]
fn test() {
    let cases: Vec<(&str, &str)> = vec![
        (
            r"3",
            "Yes
3
2 1 2
2 3 1
2 2 3"
        ),
        (
            r"4",
            "No"
        ),
        (
            r"1",
            "Yes
2
1 1
1 1"
        ),
        (
            r"100000",
            "No"
        ),
        (
            r"6",
            "No"
        ),
        (
            r"7",
            "No"
        ),
        (
            r"8",
            "No"
        ),
        (
            r"9",
            "No"
        ),
        (
            r"10",
            "No"
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
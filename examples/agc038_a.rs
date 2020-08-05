use std::io::Read;
use itertools::Itertools;

fn main() {
    let mut buf = String::new();
    std::io::stdin().read_to_string(&mut buf).unwrap();

    let answer = solve(&buf);

    println!("{}", answer);
}

fn solve(input: &str) -> String {
    let mut iterator = input.split_whitespace();

    let h: usize = iterator.next().unwrap().parse().unwrap();
    let w: usize = iterator.next().unwrap().parse().unwrap();
    let a: usize = iterator.next().unwrap().parse().unwrap();
    let b: usize = iterator.next().unwrap().parse().unwrap();

    let result = generate(h, w, a, b);
    format_result(&result)
}

fn generate(h: usize, w: usize, a: usize, b: usize) -> Vec<Vec<u8>> {
    let mut line1: Vec<u8> = vec![0; w];
    let mut line2: Vec<u8> = vec![1; w];
    for i in 0..a {
        line1[i] = 1;
        line2[i] = 0;
    }

    let mut result = Vec::new();
    for _ in 0..b {
        result.push(line1.clone());
    }
    for _ in 0..(h - b) {
        result.push(line2.clone());
    }
    result
}

fn format_result(result: &Vec<Vec<u8>>) -> String {
    result.iter().map(|line| { line.iter().join("") }).join("\n")
}

#[test]
fn test2() {
    for h in 1..=10 {
        for w in 1..=10 {
            for a in 0..=(w / 2) {
                for b in 0..=(h / 2) {
                    let result = generate(h, w, a, b);
                    println!("--- h: {}, w: {}, a: {}, b: {}", h, w, a, b);
                    if is_ok(h, w, a, b, &result) {
                        println!("OK");
                    } else {
                        println!("{}", format_result(&result));
                    }
                }
            }
        }
    }
}

fn is_ok(h: usize, w: usize, a: usize, b: usize, result: &Vec<Vec<u8>>) -> bool {
    for i in 0..h {
        let mut count_0: usize = 0;
        let mut count_1: usize = 0;
        for j in 0..w {
            if result[i][j] == 0 {
                count_0 += 1;
            } else {
                count_1 += 1;
            }
        }
        if std::cmp::min(count_0, count_1) != a {
            return false;
        }
    }
    for i in 0..w {
        let mut count_0: usize = 0;
        let mut count_1: usize = 0;
        for j in 0..h {
            if result[j][i] == 0 {
                count_0 += 1;
            } else {
                count_1 += 1;
            }
        }
        if std::cmp::min(count_0, count_1) != b {
            return false;
        }
    }
    true
}

#[test]
fn test() {
    let cases: Vec<(&str, &str)> = vec![
        (
            r"3 3 1 1",
            "100
010
001"
        ),
        (
            r"1 5 2 0",
            "01010"
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
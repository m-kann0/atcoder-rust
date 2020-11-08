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

    let mut n: usize = iterator.next().unwrap().parse().unwrap();

    let mut sum = 0;
    let mut m: Vec<usize> = vec![0; 3];
    while n > 0 {
        let i = n % 10;
        sum += i;
        m[i % 3] += 1;
        n /= 10;
    }

    if sum % 3 == 0 {
        return "0".to_string();
    }

    if sum % 3 == 1 {
        if m[1] > 0 {
            m[1] -= 1;
            if m[0] == 0 && m[1] == 0 && m[2] == 0 {
                return "-1".to_string();
            } else {
                return "1".to_string();
            }
        } else {
            m[2] -= 2;
            if m[0] == 0 && m[1] == 0 && m[2] == 0 {
                return "-1".to_string();
            } else {
                return "2".to_string();
            }
        }
    }

    if sum % 3 == 2 {
        if m[2] > 0 {
            m[2] -= 1;
            if m[0] == 0 && m[1] == 0 && m[2] == 0 {
                return "-1".to_string();
            } else {
                return "1".to_string();
            }
        } else {
            m[1] -= 2;
            if m[0] == 0 && m[1] == 0 && m[2] == 0 {
                return "-1".to_string();
            } else {
                return "2".to_string();
            }
        }
    }

    return "-1".to_string();
}

#[test]
fn test2() {
    let mut n = 3;
    for n in 1..=100 {
        let mut has_zero = false;
        let mut nn = n;
        while nn > 0 {
            let mut amari = nn % 10;
            if amari == 0 {
                has_zero = true;
                break;
            }
            nn /= 10;
        }
        if has_zero {
            continue;
        }
        println!("{}: {}", n, solve(&n.to_string()));
    }
}

#[test]
fn test() {
    let cases: Vec<(&str, &str)> = vec![
        (
            r"35",
            "1"
        ),
        (
            r"369",
            "0"
        ),
        (
            r"6227384",
            "1"
        ),
        (
            r"4622738",
            "1"
        ),
        (
            r"11",
            "-1"
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

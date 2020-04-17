use std::io::Read;

fn main() {
    let mut buf = String::new();
    std::io::stdin().read_to_string(&mut buf).unwrap();

    let answer = solve(&buf);

    println!("{}", answer);
}

fn solve(input: &str) -> String {
    let mut iterator = input.split_whitespace();

    let a: usize = iterator.next().unwrap().parse().unwrap();

    let mut l: usize = 10;
    let mut r: usize = 10000000000000000;

    while l + 1 < r {
        let m = (l + r) / 2;
        let result = check(a, m);
        match result {
            CheckResult::CORRECT => {
                return m.to_string();
            },
            CheckResult::LARGE => {
                r = m;
            },
            CheckResult::SMALL => {
                l = m;
            }
        }
    }

    return "-1".to_string();
}

fn check(mut a: usize, n: usize) -> CheckResult {
    let mut v: Vec<usize> = Vec::new();
    while a > 0 {
        v.push(a % n);
        a /= n;
    }

    let n_str = n.to_string();
    if n_str.len() > v.len() {
        return CheckResult::LARGE
    }
    if n_str.len() < v.len() {
        return CheckResult::SMALL;
    }

    for (ni, vi) in n_str.chars().zip(v.iter().rev()) {
        let ni_num: usize = ni.to_string().parse().unwrap();
        if ni_num > *vi {
            return CheckResult::LARGE;
        }
        if ni_num < *vi {
            return CheckResult::SMALL;
        }
    }

    return CheckResult::CORRECT;
}

enum CheckResult {
    LARGE, SMALL, CORRECT
}

#[test]
fn test() {
    let cases: Vec<(&str, &str)> = vec![
        (
            r"49",
            "23"
        ),
        (
            r"999999999999999",
            "-1"
        ),
        (
            r"10000000000000000",
            "10000"
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
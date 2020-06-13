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
    let k: usize = iterator.next().unwrap().parse().unwrap();
    let mut a: Vec<isize> = (0..n).map(|_| iterator.next().unwrap().parse().unwrap()).collect();

    for _ in 0..k {
        let mut b: Vec<isize> = vec![0; n];
        for i in 0..n {
            let l: isize = max(0, i as isize - a[i] as isize);
            let r: isize = min(n as isize - 1, i as isize + a[i] as isize);
            b[l as usize] += 1;
            if r + 1 < n as isize {
                b[r as usize + 1] -= 1;
            }
        }
        for i in 1..n {
            b[i] += b[i - 1];
        }
        let mut all_n = true;
        for i in 0..n {
            a[i] = b[i];
            if a[i] != n as isize {
                all_n = false;
            }
        }
        if all_n {
            break;
        }
    }

    let mut result = String::new();
    for i in 0..n {
        result.push_str(&format!("{} ", a[i]));
    }
    return result.trim().to_string();
}

#[test]
fn test() {
    let cases: Vec<(&str, &str)> = vec![
        (
            r"5 1
1 0 0 1 0",
            "1 2 2 1 2"
        ),
        (
            r"5 2
1 0 0 1 0",
            "3 3 4 4 3"
        ),
        (
            r"5 3
1 0 0 1 0",
            "4 5 5 5 4"
        ),
        (
            r"5 4
1 0 0 1 0",
            "5 5 5 5 5"
        ),
        (
            r"5 5
1 0 0 1 0",
            "5 5 5 5 5"
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
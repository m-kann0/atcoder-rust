use std::io::Read;
use std::collections::HashSet;
use std::cmp::min;

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
    let x: usize = iterator.next().unwrap().parse().unwrap();
    let a: Vec<usize> = (0..m).map(|_| iterator.next().unwrap().parse().unwrap()).collect();

    let mut a1 = 0;
    let mut a2 = 0;
    for i in 0..m {
        if a[i] < x {
            a1 += 1;
        } else {
            a2 += 1;
        }
    }
    min(a1, a2).to_string()
}

#[test]
fn test() {
    let cases: Vec<(&str, &str)> = vec![
        (
            r"5 3 3
1 2 4",
            "1"
        ),
        (
            r"7 3 2
4 5 6",
            "0"
        ),
        (
            r"10 7 5
1 2 3 4 6 8 9",
            "3"
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
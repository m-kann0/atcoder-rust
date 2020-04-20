use std::io::Read;
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
    let a: Vec<isize> = (0..n)
        .map(|_| iterator.next().unwrap().parse().unwrap())
        .collect();

    let mut min_cost = 1_000_000_000;
    for i in -100..101 {
        let mut cost = 0;
        for j in 0..n {
            cost += (a[j] - i).abs().pow(2);
        }
        min_cost = min(min_cost, cost);
    }

    return format!("{}", min_cost);
}

#[test]
fn test() {
    let cases: Vec<(&str, &str)> = vec![
        (
            r"2
4 8",
            "8"
        ),
        (
            r"3
1 1 3",
            "3"
        ),
        (
            r"3
4 2 5",
            "5"
        ),
        (
            r"4
-100 -100 -100 -100",
            "0"
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
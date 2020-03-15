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

    let n: usize = iterator.next().unwrap().parse().unwrap();
    let mut max_a: usize = 0;
    let mut min_b: usize = 1_000_000_001;

    for _ in 0..n {
        max_a = max(max_a, iterator.next().unwrap().parse().unwrap());
        min_b = min(min_b, iterator.next().unwrap().parse().unwrap());
    }

    return format!("{}", max_a + min_b);
}

#[test]
fn test() {
    let cases: Vec<(&str, &str)> = vec![
        (
            r"3
4 7
2 9
6 2",
            "8"
        ),
        (
            r"5
1 10
3 6
5 2
4 4
2 8",
            "7"
        ),
        (
            r"2
1 1000000000
1000000000 1",
            "1000000001"
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
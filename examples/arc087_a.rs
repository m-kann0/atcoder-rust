use std::io::Read;
use std::collections::HashMap;

fn main() {
    let mut buf = String::new();
    std::io::stdin().read_to_string(&mut buf).unwrap();

    let answer = solve(&buf);

    println!("{}", answer);
}

fn solve(input: &str) -> String {
    let mut iterator = input.split_whitespace();

    let n: usize = iterator.next().unwrap().parse().unwrap();
    let mut counts: HashMap<usize, usize> = HashMap::new();
    for _ in 0..n {
        let ai: usize = iterator.next().unwrap().parse().unwrap();
        *counts.entry(ai).or_insert(0) += 1;
    }

    let mut ans = 0;
    for (k, v) in counts {
        if k > v {
            ans += v;
        } else {
            ans += v - k;
        }
    }
    return format!("{}", ans);
}

#[test]
fn test() {
    let cases: Vec<(&str, &str)> = vec![
        (
            r"4
3 3 3 3",
            "1"
        ),
        (
            r"5
2 4 1 4 2",
            "2"
        ),
        (
            r"6
1 2 2 3 3 3",
            "0"
        ),
        (
            r"1
1000000000",
            "1"
        ),
        (
            r"8
2 7 1 8 2 8 1 8",
            "5"
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
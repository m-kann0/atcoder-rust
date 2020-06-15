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
    let a: usize = iterator.next().unwrap().parse().unwrap();
    let b: usize = iterator.next().unwrap().parse().unwrap();
    let x: Vec<usize> = (0..n).map(|_| iterator.next().unwrap().parse().unwrap()).collect();

    let mut ans = 0;
    for i in 1..n {
        ans += min((x[i] - x[i - 1]) * a,b);
    }
    return ans.to_string();
}

#[test]
fn test() {
    let cases: Vec<(&str, &str)> = vec![
        (
            r"4 2 5
1 2 5 7",
            "11"
        ),
        (
            r"7 1 100
40 43 45 105 108 115 124",
            "84"
        ),
        (
            r"7 1 2
24 35 40 68 72 99 103",
            "12"
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
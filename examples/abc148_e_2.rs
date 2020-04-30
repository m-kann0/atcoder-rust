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

    if n % 2 != 0 {
        return "0".to_string();
    }

    let ans = min(g2(n), g5(n));
    return ans.to_string();
}

fn g2(n: usize) -> usize {
    return g(n / 2, 2) + n / 2;
}

fn g5(n: usize) -> usize {
    return g(n / 2, 5);
}

fn g(n: usize, p: usize) -> usize {
    if n == 0 {
        return 0;
    }
    return n / p + g(n / p, p);
}

#[test]
fn test() {
    let cases: Vec<(&str, &str)> = vec![
        (
            r"12",
            "1"
        ),
        (
            r"5",
            "0"
        ),
        (
            r"1000000000000000000",
            "124999999999999995"
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
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

    let n: u64 = iterator.next().unwrap().parse().unwrap();
    let a: u64 = iterator.next().unwrap().parse().unwrap();
    let b: u64 = iterator.next().unwrap().parse().unwrap();

    if a == 0 {
        return "0".to_string();
    }

    let ans: u64 = a * (n / (a + b)) + min(a, n % (a + b));

    return ans.to_string();
}

#[test]
fn test() {
    let cases: Vec<(&str, &str)> = vec![
        (
            r"8 3 4",
            "4"
        ),
        (
            r"8 0 4",
            "0"
        ),
        (
            r"6 2 4",
            "2"
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
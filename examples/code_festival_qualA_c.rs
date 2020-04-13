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
    let b: usize = iterator.next().unwrap().parse().unwrap();

    let ans: usize = count_leap_year(b) - count_leap_year(a - 1);
    return ans.to_string();
}

// 0年～year年までのうるう年の回数を数える
fn count_leap_year(year: usize) -> usize {
    return year / 4 + year / 400 - year / 100;
}

#[test]
fn test() {
    let cases: Vec<(&str, &str)> = vec![
        (
            r"1988 2014",
            "7"
        ),
        (
            r"997 1003",
            "0"
        ),
        (
            r"1 2000000000",
            "485000000"
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
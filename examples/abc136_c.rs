use std::io::Read;
use std::str::{SplitWhitespace, FromStr};

fn main() {
    let mut buf = String::new();
    std::io::stdin().read_to_string(&mut buf).unwrap();

    let answer = solve(&buf);

    println!("{}", answer);
}

fn solve(input: &str) -> String {
    let mut iterator = input.split_whitespace();

    let n: usize = iterator.next().unwrap().parse().unwrap();

    let mut prepre: usize = 0;
    let mut pre: usize = 0;
    for _i in 0..n {
        let mut current: usize = iterator.next().unwrap().parse().unwrap();
        if prepre > current {
            return String::from("No")
        }
        if pre > current {
            return String::from("No")
        }
        if current > pre {
            current -= 1;
        }
        prepre = pre;
        pre = current;
    }

    return String::from("Yes");
}

#[test]
fn test() {
    let cases: Vec<(&str, &str)> = vec![
        (
            r"5
1 2 1 1 3",
            "Yes"
        ),
        (
            r"4
1 3 2 1",
            "No"
        ),
        (
            r"5
1 2 3 4 5",
            "Yes"
        ),
        (
            r"1
1000000000",
            "Yes"
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

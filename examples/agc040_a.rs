use std::io::Read;
use std::cmp::max;

fn main() {
    let mut buf = String::new();
    std::io::stdin().read_to_string(&mut buf).unwrap();

    let answer = solve(&buf);

    println!("{}", answer);
}

fn solve(input: &str) -> String {
    let mut iterator = input.split_whitespace();

    let s: Vec<char> = iterator.next().unwrap().chars().collect();

    let n: usize = s.len() + 1;
    let mut a: Vec<usize> = vec![0; n];

    for i in 0..s.len() {
        if s[i] == '<' {
            a[i + 1] = max(a[i + 1], a[i] + 1);
        }
    }

    for i in (0..s.len()).rev() {
        if s[i] == '>' {
            a[i] = max(a[i], a[i + 1] + 1);
        }
    }

    let ans: usize = a.iter().sum();
    return ans.to_string();
}

#[test]
fn test() {
    let cases: Vec<(&str, &str)> = vec![
        (
            r"<>>",
            "3"
        ),
        (
            r"<>>><<><<<<<>>><",
            "28"
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
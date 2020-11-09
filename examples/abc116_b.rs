use std::io::Read;
use std::collections::HashSet;

fn main() {
    let mut buf = String::new();
    std::io::stdin().read_to_string(&mut buf).unwrap();

    let answer = solve(&buf);

    println!("{}", answer);
}

fn solve(input: &str) -> String {
    let mut iterator = input.split_whitespace();

    let s: usize = iterator.next().unwrap().parse().unwrap();

    let mut ai = s;
    let mut a = HashSet::new();
    let mut m = 1;
    a.insert(s);
    while m <= 1000000 {
        m += 1;
        ai = if ai % 2 == 0 {
            ai / 2
        } else {
            ai * 3 + 1
        };
        if a.contains(&ai) {
            break;
        } else {
            a.insert(ai);
        }
    }
    m.to_string()
}

#[test]
fn test() {
    let cases: Vec<(&str, &str)> = vec![
        (
            r"8",
            "5"
        ),
        (
            r"7",
            "18"
        ),
        (
            r"54",
            "114"
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
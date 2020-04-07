use std::io::Read;

fn main() {
    let mut buf = String::new();
    std::io::stdin().read_to_string(&mut buf).unwrap();

    let answer = solve(&buf);

    println!("{}", answer);
}

fn solve(input: &str) -> String {
    let mut iterator = input.split_whitespace();

    let n: usize = iterator.next().unwrap().parse().unwrap();
    let k: usize = iterator.next().unwrap().parse().unwrap();
    let s: usize = iterator.next().unwrap().parse().unwrap();

    let mut result = String::new();

    for i in 0..n {
        if i < k {
            result.push_str(&format!("{} ", s));
        } else {
            if s == 1_000_000_000 {
                result.push_str(&format!("{} ", 1));
            } else {
                result.push_str(&format!("{} ", 1_000_000_000));
            }
        }
    }

    return result.trim().to_string();
}

#[test]
fn test() {
    let cases: Vec<(&str, &str)> = vec![
        (
            r"4 2 3",
            "1 2 3 4"
        ),
        (
            r"5 3 100",
            "50 50 50 30 70"
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
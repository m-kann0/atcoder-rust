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
    let r: usize = iterator.next().unwrap().parse().unwrap();
    let n: usize = iterator.next().unwrap().parse().unwrap();

    let mut current = a;
    for _ in 0..(n - 1) {
        if current * r > 1_000_000_000 {
            return "large".to_string();
        }
        current *= r;
    }

    if current > 1_000_000_000 {
        return "large".to_string();
    }
    return current.to_string();
}

#[test]
fn test() {
    let cases: Vec<(&str, &str)> = vec![
        (
            r"2 3 4",
            "54"
        ),
        (
            r"4 3 21",
            "large"
        ),
        (
            r"12 34 5",
            "16036032"
        ),
        (
            r"1 1 1",
            "1"
        ),
        (
            r"1000000000 1000000000 1000000000",
            "large"
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
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

    let s: usize = (1..=n).sum();

    if s <= 1 {
        return "BOWWOW".to_string();
    }

    let mut i = 2;
    while i * i <= s {
        if s % i == 0 {
            return "BOWWOW".to_string();
        }
        i += 1;
    }

    return "WANWAN".to_string();
}

#[test]
fn test() {
    let cases: Vec<(&str, &str)> = vec![
        (
            r"2",
            "WANWAN"
        ),
        (
            r"5",
            "BOWWOW"
        ),
        (
            r"1",
            "BOWWOW"
        ),
        (
            r"999",
            "BOWWOW"
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
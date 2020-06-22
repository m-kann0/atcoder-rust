use std::io::Read;

fn main() {
    let mut buf = String::new();
    std::io::stdin().read_to_string(&mut buf).unwrap();

    let answer = solve(&buf);

    println!("{}", answer);
}

fn solve(input: &str) -> String {
    let mut iterator = input.split_whitespace();

    let k: isize = iterator.next().unwrap().parse().unwrap();
    let x: isize = iterator.next().unwrap().parse().unwrap();

    let mut result = String::new();
    for i in (x - k + 1)..=(x + k - 1) {
        if i >= -1000000 && i <= 1000000 {
            result.push_str(&format!("{} ", i));
        }
    }

    return result.trim().to_string();
}

#[test]
fn test() {
    let cases: Vec<(&str, &str)> = vec![
        (
            r"3 7",
            "5 6 7 8 9"
        ),
        (
            r"4 0",
            "-3 -2 -1 0 1 2 3"
        ),
        (
            r"1 100",
            "100"
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
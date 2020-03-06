use std::io::Read;

fn main() {
    let mut buf = String::new();
    std::io::stdin().read_to_string(&mut buf).unwrap();

    let answer = solve(&buf);

    println!("{}", answer);
}

fn solve(input: &str) -> String {
    let mut iterator = input.split_whitespace();

    let x: i64 = iterator.next().unwrap().parse().unwrap();
    let y: i64 = iterator.next().unwrap().parse().unwrap();

    let ans =
        if x == y {
            0
        } else if x > 0 && y == 0 {
            x + 1
        } else if x < 0 && y == 0 {
            -x
        } else if x >= 0 && y >= 0 {
            if x <= y {
                y - x
            } else {
                x - y + 2
            }
        } else if x >= 0 && y < 0 {
            if x <= -y {
                (-y) - x + 1
            } else {
                x - (-y) + 1
            }
        } else if x < 0 && y >= 0 {
            if -x <= y {
                y - (-x) + 1
            } else {
                (-x) - y + 1
            }
        } else {
            if -x <= -y {
                (-y) - (-x) + 2
            } else {
                (-x) - (-y)
            }
        };

    return ans.to_string();
}

#[test]
fn test() {
    let cases: Vec<(&str, &str)> = vec![
        (
            r"10 20",
            "10"
        ),
        (
            r"10 5",
            "7"
        ),
        (
            r"10 10",
            "0"
        ),
        (
            r"10 -20",
            "11"
        ),
        (
            r"10 -5",
            "6"
        ),
        (
            r"10 -10",
            "1"
        ),
        (
            r"-10 20",
            "11"
        ),
        (
            r"-10 5",
            "6"
        ),
        (
            r"-10 10",
            "1"
        ),
        (
            r"-10 -20",
            "12"
        ),
        (
            r"-10 -5",
            "5"
        ),
        (
            r"-10 -10",
            "0"
        ),
        (
            r"10 0",
            "11"
        ),
        (
            r"-10 0",
            "10"
        ),
        (
            r"0 10",
            "10"
        ),
        (
            r"0 -10",
            "11"
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
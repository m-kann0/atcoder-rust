use std::io::Read;

fn main() {
    let mut buf = String::new();
    std::io::stdin().read_to_string(&mut buf).unwrap();

    let answer = solve(&buf);

    println!("{}", answer);
}

fn solve(input: &str) -> String {
    let mut iterator = input.split_whitespace();

    let x: usize = iterator.next().unwrap().parse().unwrap();

    if x <= 6 {
        return "1".to_string();
    }

    if x <= 11 {
        return "2".to_string();
    }

    let mut ans = x / 11 * 2;
    if x % 11 != 0 {
        if x % 11 > 6 {
            ans += 2;
        } else {
            ans += 1;
        }
    }
    return ans.to_string();
}

#[test]
fn test() {
    let cases: Vec<(&str, &str)> = vec![
        (
            r"7",
            "2"
        ),
        (
            r"149696127901",
            "27217477801"
        ),
        (
            r"6",
            "1"
        ),
        (
            r"11",
            "2"
        ),
        (
            r"12",
            "3"
        ),
        (
            r"17",
            "3"
        ),
        (
            r"18",
            "4"
        ),
        (
            r"22",
            "4"
        ),
        (
            r"23",
            "5"
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
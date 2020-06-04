use std::io::Read;

fn main() {
    let mut buf = String::new();
    std::io::stdin().read_to_string(&mut buf).unwrap();

    let answer = solve(&buf);

    println!("{}", answer);
}

fn solve(input: &str) -> String {
    let mut iterator = input.split_whitespace();

    let n: u32 = iterator.next().unwrap().parse().unwrap();
    let s: Vec<char> = iterator.next().unwrap().chars().collect();
    let mut result = String::new();
    for c in s {
        let mut u = c as u32 + n;
        if u > 'Z' as u32 {
            u = 'A' as u32 + u - 'Z' as u32 - 1;
        }
        result.push(std::char::from_u32(u).unwrap());
    }
    return result;
}

#[test]
fn test() {
    let cases: Vec<(&str, &str)> = vec![
        (
            r"2
ABCXYZ",
            "CDEZAB"
        ),
        (
            r"0
ABCXYZ",
            "ABCXYZ"
        ),
        (
            r"13
ABCDEFGHIJKLMNOPQRSTUVWXYZ",
            "NOPQRSTUVWXYZABCDEFGHIJKLM"
        ),
        (
            r"26
ABCXYZ",
            "ABCXYZ"
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
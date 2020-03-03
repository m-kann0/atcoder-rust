use std::io::Read;

fn main() {
    let mut buf = String::new();
    std::io::stdin().read_to_string(&mut buf).unwrap();

    let answer = solve(&buf);

    println!("{}", answer);
}

fn solve(input: &str) -> String {
    let mut iterator = input.split_whitespace();

    let s: &str = iterator.next().unwrap();

    let mut result = String::new();

    let mut character: char = ' ';
    let mut count: u32 = 0;
    for c in s.chars() {
        if c != character {
            result.push(character);
            result += &count.to_string();
            character = c;
            count = 1;
        } else {
            count += 1;
        }
    }
    result.push(character);
    result += &count.to_string();

    return result.replace(" 0", "");
}

#[test]
fn test() {
    let cases: Vec<(&str, &str)> = vec![
        (
            r"aabbbaad",
            "a2b3a2d1"
        ),
        (
            r"aabbbbbbbbbbbbxyza",
            "a2b12x1y1z1a1"
        ),
        (
            r"edcba",
            "e1d1c1b1a1"
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
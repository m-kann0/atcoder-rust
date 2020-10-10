use std::io::Read;

fn main() {
    let mut buf = String::new();
    std::io::stdin().read_to_string(&mut buf).unwrap();

    let answer = solve(&buf);

    println!("{}", answer);
}

fn solve(input: &str) -> String {
    let mut iterator = input.split_whitespace();

    let s: Vec<isize> = iterator.next().unwrap().chars()
        .map(|c| c.to_digit(10).unwrap() as isize)
        .collect();

    for bit in 0..(1 << 3) {
        let mut sum = s[0];
        let mut result = String::new();
        result.push_str(&format!("{}", s[0]));
        for j in 0..3 {
            if bit & 1 << j > 0 {
                sum += s[j + 1];
                result.push_str(&format!("+{}", s[j + 1]));
            } else {
                sum -= s[j + 1];
                result.push_str(&format!("-{}", s[j + 1]));
            }
        }

        if sum == 7 {
            result.push_str(&format!("=7"));
            return result;
        }
    }

    return String::new();
}

#[test]
fn test() {
    let cases: Vec<(&str, &str)> = vec![
        (
            r"1222",
            "1+2+2+2=7"
        ),
        (
            r"0290",
            "0-2+9+0=7"
        ),
        (
            r"3242",
            "3+2+4-2=7"
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
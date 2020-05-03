use std::io::Read;

fn main() {
    let mut buf = String::new();
    std::io::stdin().read_to_string(&mut buf).unwrap();

    let answer = solve(&buf);

    println!("{}", answer);
}

fn solve(input: &str) -> String {
    let mut iterator = input.split_whitespace();

    let mut s: Vec<char> = iterator.next().unwrap().chars().collect();

    for i in 0..s.len() {
        if s[i] != '2' && s[i] != '5' {
            s[i] = '_';
        }
        if s[i] == '2' && i + 1 < s.len() && s[i + 1] != '5' {
            s[i] = '_';
        }
        if s[i] == '5' && i > 0 && s[i - 1] != '2' {
            s[i] = '_';
        }
        if s[i] == '2' && i + 1 == s.len() {
            s[i] = '_';
        }
        if s[i] == '5' && i == 0 {
            s[i] = '_';
        }
    }
    s.push('_');

    let mut ans: usize = 0;
    let mut length: usize = 0;
    for i in 1..s.len() {
        if s[i] == '5' && s[i - 1] == '2' {
            length += 1;
        } else if s[i] == '_' {
            ans += length * (length + 1) / 2;
            length = 0;
        }
    }
    return ans.to_string();
}

#[test]
fn test() {
    let cases: Vec<(&str, &str)> = vec![
        (
            r"2525",
            "3"
        ),
        (
            r"1251251252525",
            "8"
        ),
        (
            r"25225",
            "2"
        ),
        (
            r"252252252252252252",
            "6"
        ),
        (
            r"20061212",
            "0"
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
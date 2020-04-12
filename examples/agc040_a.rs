use std::io::Read;

fn main() {
    let mut buf = String::new();
    std::io::stdin().read_to_string(&mut buf).unwrap();

    let answer = solve(&buf);

    println!("{}", answer);
}

fn solve(input: &str) -> String {
    let mut iterator = input.split_whitespace();

    let s: Vec<char> = iterator.next().unwrap().chars().collect();

    let mut sum: Vec<usize> = vec![0; s.len() + 1];
    for i in 1..(s.len() + 1) {
        sum[i] = sum[i - 1] + i;
    }

    let mut parts: Vec<String> = Vec::new();
    let mut part = String::new();
    let mut prev: char = ' ';
    for &c in &s {
        if c != prev {
            if prev != ' ' {
                parts.push(part.clone());
            }
            part = String::new();
        }
        part.push(c);
        prev = c;
    }
    parts.push(part);

    let mut ans = 0;
    for i in 0..parts.len() {
        let part = &parts[i];
        if part.starts_with('<') {
            ans += sum[part.len()];
        } else {
            if i == 0 {
                ans += sum[part.len()];
            } else if part.len() <= parts[i - 1].len() {
                ans += sum[part.len() - 1];
            } else {
                ans -= parts[i - 1].len();
                ans += sum[part.len()];
            }
        }
    }
    return ans.to_string();
}

#[test]
fn test() {
    let cases: Vec<(&str, &str)> = vec![
        (
            r"<>>",
            "3"
        ),
        (
            r"<>>><<><<<<<>>><",
            "28"
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
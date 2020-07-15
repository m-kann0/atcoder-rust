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

    let mut t: Vec<char> = Vec::new();
    let mut i = 0;
    while i < s.len() {
        if s[i] == 'B' && i + 1 < s.len() && s[i + 1] == 'C' {
            t.push('D');
            i += 2;
        } else {
            t.push(s[i]);
            i += 1;
        }
    }

    let mut counts: Vec<usize> = vec![0; t.len()];
    if t[t.len() - 1] == 'D' {
        counts[t.len() - 1] = 1;
    }
    if t.len() < 2 {
        return "0".to_string();
    }
    for i in (0..=(t.len() - 2)).rev() {
        if t[i] == 'D' {
            counts[i] = counts[i + 1] + 1;
        } else if t[i] == 'A' {
            counts[i] = counts[i + 1];
        } else {
            counts[i] = 0;
        }
    }

    let mut ans = 0;
    for i in 0..t.len() {
        if t[i] == 'A' {
            ans += counts[i];
        }
    }
    ans.to_string()
}

#[test]
fn test() {
    let cases: Vec<(&str, &str)> = vec![
        (
            r"ABCABC",
            "3"
        ),
        (
            r"C",
            "0"
        ),
        (
            r"ABCACCBABCBCAABCB",
            "6"
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
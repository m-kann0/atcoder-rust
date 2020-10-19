use std::io::Read;

fn main() {
    let mut buf = String::new();
    std::io::stdin().read_to_string(&mut buf).unwrap();

    let answer = solve(&buf);

    println!("{}", answer);
}

fn solve(input: &str) -> String {
    let mut iterator = input.split_whitespace();

    let t: usize = iterator.next().unwrap().parse().unwrap();

    let atcoder = "atcoder";

    let mut result = String::new();
    for _ in 0..t {
        let s = iterator.next().unwrap();
        if s.chars().all(|c| c == 'a') {
            result.push_str(&format!("-1\n"));
            continue;
        }
        if s > atcoder {
            result.push_str(&format!("0\n"));
            continue;
        }
        let s: Vec<char> = s.chars().collect();

        for i in 0..s.len() {
            if s[i] != 'a' {
                if i >= 2 && s[i] > 't' {
                    result.push_str(&format!("{}\n", i - 1));
                } else {
                    result.push_str(&format!("{}\n", i));
                }
                break;
            }
        }
    }
    result.trim().to_string()
}

#[test]
fn test() {
    let cases: Vec<(&str, &str)> = vec![
        (
            r"11
atcodeer
codeforces
aaa
atcoder
at
atcodera
aab
aaaaaaab
atcd
aau
aaad",
            "1
0
-1
1
1
0
2
7
1
1
3"
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
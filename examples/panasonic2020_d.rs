use std::io::Read;
use std::collections::HashSet;

fn main() {
    let mut buf = String::new();
    std::io::stdin().read_to_string(&mut buf).unwrap();

    let answer = solve(&buf);

    println!("{}", answer);
}

fn solve(input: &str) -> String {
    let mut iterator = input.split_whitespace();

    let n: usize = iterator.next().unwrap().parse().unwrap();

    if n == 1 {
        return "a".to_string();
    }

    let alphabets = vec!['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j'];

    let mut prev: Vec<String> = vec!["a".to_string()];
    for i in 2..=n {
        let mut next: Vec<String> = Vec::new();
        for s in prev {
            let dim = s.chars().collect::<HashSet<char>>().len();
            for j in 0..(dim + 1) {
                next.push(format!("{}{}", s, alphabets[j]));
            }
        }
        prev = next;
    }

    let mut result = String::new();
    for s in prev {
        result.push_str(&format!("{}\n", s));
    }
    result.trim().to_string()
}

#[test]
fn test() {
    let cases: Vec<(&str, &str)> = vec![
        (
            r"1",
            "a"
        ),
        (
            r"2",
            "aa
ab"
        ),
        (
            r"3",
            "aaa
aab
aba
abb
abc"
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
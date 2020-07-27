use std::io::Read;
use std::cmp::max;

fn main() {
    let mut buf = String::new();
    std::io::stdin().read_to_string(&mut buf).unwrap();

    let answer = solve(&buf);

    println!("{}", answer);
}

fn solve(input: &str) -> String {
    let mut iterator = input.split_whitespace();

    let n: usize = iterator.next().unwrap().parse().unwrap();

    let mut results = Vec::new();
    dfs(n, &mut results, String::new(), -1);
    results.join("\n")
}

fn dfs(n: usize, results: &mut Vec<String>, s: String, mx: isize) {
    if s.len() == n {
        results.push(s);
        return;
    }

    let alphabets = vec!['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j'];
    for i in 0..=(mx + 1) {
        dfs(n, results, format!("{}{}", s, alphabets[i as usize]), max(mx, i));
    }
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
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

    let s = iterator.next().unwrap();
    let k: usize = iterator.next().unwrap().parse().unwrap();

    if k > s.len() {
        return "0".to_string();
    }

    let mut set: HashSet<String> = HashSet::new();
    for i in 0..(s.len() - k + 1) {
        set.insert(s[i..(i + k)].to_string());
    }

    return set.len().to_string();
}

#[test]
fn test() {
    let cases: Vec<(&str, &str)> = vec![
        (
            r"abcabc
2",
            "3"
        ),
        (
            r"aaaaa
1",
            "1"
        ),
        (
            r"hello
10",
            "0"
        ),
        (
            r"abcde
3",
            "3"
        ),
        (
            r"abcde
5",
            "1"
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
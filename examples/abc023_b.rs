use std::io::Read;

fn main() {
    let mut buf = String::new();
    std::io::stdin().read_to_string(&mut buf).unwrap();

    let answer = solve(&buf);

    println!("{}", answer);
}

fn solve(input: &str) -> String {
    let mut iterator = input.split_whitespace();

    let _n: usize = iterator.next().unwrap().parse().unwrap();
    let s = iterator.next().unwrap().to_string();

    let mut t = String::from("b");
    let mut count = 0;

    while t.len() <= s.len() {
        if t == s {
            return format!("{}", count);
        }

        count += 1;

        if count % 3 == 1 {
            t = format!("a{}c", t);
        } else if count % 3 == 2 {
            t = format!("c{}a", t);
        } else {
            t = format!("b{}b", t);
        }
    }

    "-1".to_string()
}

#[test]
fn test() {
    let cases: Vec<(&str, &str)> = vec![
        (
            r"3
abc",
            "1"
        ),
        (
            r"6
abcabc",
            "-1"
        ),
        (
            r"7
atcoder",
            "-1"
        ),
        (
            r"19
bcabcabcabcabcabcab",
            "9"
        ),
        (
            r"1
b",
            "0"
        ),
        (
            r"1
a",
            "-1"
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
use std::io::Read;

fn main() {
    let mut buf = String::new();
    std::io::stdin().read_to_string(&mut buf).unwrap();

    let answer = solve(&buf);

    println!("{}", answer);
}

fn solve(input: &str) -> String {
    let mut iterator = input.split_whitespace();

    let n: usize = iterator.next().unwrap().parse().unwrap();
    let k: usize = iterator.next().unwrap().parse().unwrap();
    let mut s: Vec<char> = iterator.next().unwrap().chars().collect();

    let mut prev_end = 0;
    for _ in 0..k {
        let mut begin = 0;
        for i in (prev_end + 1)..n {
            if s[i - 1] != s[i] {
                begin = i;
                break;
            }
        }
        if begin == 0 {
            return (n - 1).to_string();
        }
        let mut end = begin;
        for i in (begin + 1)..n {
            if s[i - 1] == s[i] {
                end = i;
            } else {
                break;
            }
        }
        for i in begin..=end {
            if s[i] == 'L' {
                s[i] = 'R';
            } else {
                s[i] = 'L';
            }
        }
        prev_end = end;
    }

    let mut h: usize = 0;
    for i in 1..n {
        if s[i - 1] == s[i] {
            h += 1;
        }
    }
    h.to_string()
}

#[test]
fn test() {
    let cases: Vec<(&str, &str)> = vec![
        (
            r"6 1
LRLRRL",
            "3"
        ),
        (
            r"13 3
LRRLRLRRLRLLR",
            "9"
        ),
        (
            r"10 1
LLLLLRRRRR",
            "9"
        ),
        (
            r"9 2
RRRLRLRLL",
            "7"
        ),
        (
            r"10 100
LLLLLLLLLL",
            "9"
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
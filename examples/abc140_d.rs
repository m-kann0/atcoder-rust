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
    let s: Vec<char> = iterator.next().unwrap().chars().collect();

    let mut score: usize = 0;
    for i in 1..n {
        if s[i - 1] == s[i] {
            score += 1;
        }
    }
    for _ in 0..k {
        if score + 2 <= n - 1 {
            score += 2;
        } else if score + 1 <= n - 1 {
            score += 1;
        } else {
            break;
        }
    }
    score.to_string()
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
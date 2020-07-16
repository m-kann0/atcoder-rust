use std::io::Read;
use std::cmp::min;

fn main() {
    let mut buf = String::new();
    std::io::stdin().read_to_string(&mut buf).unwrap();

    let answer = solve(&buf);

    println!("{}", answer);
}

fn solve(input: &str) -> String {
    let mut iterator = input.split_whitespace();

    let n: usize = iterator.next().unwrap().parse().unwrap();
    let r: usize = iterator.next().unwrap().parse().unwrap();
    let mut s: Vec<char> = iterator.next().unwrap().chars().collect();

    let mut last_not_painted = None;
    for i in 0..n {
        if s[i] == '.' {
            last_not_painted = Some(i);
        }
    }

    if last_not_painted.is_none() {
        return "0".to_string();
    }

    let last_not_painted = last_not_painted.unwrap();

    let mut ans: usize = 0;
    let mut i = 0;
    while s.iter().filter(|it| **it == '.').count() > 0 {
        let before = i;
        while i < n && s[i] == 'o' && i + r - 1 < last_not_painted {
            i += 1;
        }
        if i >= n {
            break;
        }
        ans += i - before;
        for j in i..min(i + r, n) {
            if s[j] == '.' {
                s[j] = 'o';
            }
        }
        ans += 1;
    }
    return ans.to_string();
}

#[test]
fn test() {
    let cases: Vec<(&str, &str)> = vec![
        (
            r"7 3
...o.o.",
            "6"
        ),
        (
            r"8 4
...o.ooo",
            "3"
        ),
        (
            r"4 4
oooo",
            "0"
        ),
        (
            r"5 5
o.ooo",
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
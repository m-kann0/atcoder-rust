use std::io::Read;
use std::collections::VecDeque;
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
    let k: usize = iterator.next().unwrap().parse().unwrap();
    let s: Vec<char> = iterator.next().unwrap().chars().collect();

    let mut q1: VecDeque<(char, usize)> = VecDeque::new();
    let mut current = (s[0], 1);
    for i in 1..n {
        if s[i] != current.0 {
            q1.push_back(current);
            current = (s[i], 1);
        } else {
            current.1 += 1;
        }
    }
    q1.push_back(current);

    let mut count: usize = 0;
    let mut score: usize = 0;
    let mut q2: VecDeque<(char, usize)> = VecDeque::new();
    while count < k && !q1.is_empty() {
        let current = q1.pop_front().unwrap();
        if current.0 == '0' {
            count += 1;
        }
        score += current.1;
        q2.push_back(current);
    }
    if !q1.is_empty() {
        let current = q1.pop_front().unwrap();
        score += current.1;
        q2.push_back(current);
    }

    let mut ans = score;
    while !q1.is_empty() {
        if let Some(removed) = q2.pop_front() {
            score -= removed.1;
            if removed.0 == '1' {
                if let Some(removed2) = q2.pop_front() {
                    score -= removed2.1;
                }
            }
        }
        let next = q1.pop_front().unwrap();
        score += next.1;
        q2.push_back(next);
        if !q1.is_empty() {
            let next = q1.pop_front().unwrap();
            score += next.1;
            q2.push_back(next);
        }
        ans = max(ans, score);
    }
    ans.to_string()
}

#[test]
fn test() {
    let cases: Vec<(&str, &str)> = vec![
        (
            r"5 1
00010",
            "4"
        ),
        (
            r"14 2
11101010110011",
            "8"
        ),
        (
            r"1 1
1",
            "1"
        ),
        (
            r"5 1
11111",
            "5"
        ),
        (
            r"5 3
00000",
            "5"
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
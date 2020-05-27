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

    let s: Vec<char> = iterator.next().unwrap().chars().collect();
    let t: Vec<char> = iterator.next().unwrap().chars().collect();

    let p: Vec<Vec<usize>> = destruct(&s);
    let q: Vec<Vec<usize>> = destruct(&t);

    if p.len() != q.len() {
        return "No".to_string();
    }

    for i in 0..p.len() {
        let pi = &p[i];
        let qi = &q[i];
        if pi.len() != qi.len() {
            return "No".to_string();
        }
        for j in 0..pi.len() {
            if pi[j] != qi[j] {
                return "No".to_string();
            }
        }
    }

    return "Yes".to_string()
}

fn destruct(s: &Vec<char>) -> Vec<Vec<usize>> {
    let mut result: Vec<Vec<usize>> = Vec::new();

    let mut checked: HashSet<char> = HashSet::new();
    for i in 0..s.len() {
        let c = s[i];
        if checked.contains(&c) {
            continue;
        }
        let mut v: Vec<usize> = Vec::new();
        for j in 0..s.len() {
            if s[j] == c {
                v.push(j);
            }
        }
        result.push(v);
        checked.insert(c);
    }

    result.sort();
    return result;
}

#[test]
fn test() {
    let cases: Vec<(&str, &str)> = vec![
        (
            r"azzel
apple",
            "Yes"
        ),
        (
            r"chokudai
redcoder",
            "No"
        ),
        (
            r"abcdefghijklmnopqrstuvwxyz
ibyhqfrekavclxjstdwgpzmonu",
            "Yes"
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
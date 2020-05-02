use std::io::Read;

fn main() {
    let mut buf = String::new();
    std::io::stdin().read_to_string(&mut buf).unwrap();

    let answer = solve(&buf);

    println!("{}", answer);
}

fn solve(input: &str) -> String {
    let mut iterator = input.split_whitespace();

    let s_: Vec<char> = iterator.next().unwrap().chars().collect();
    let t: Vec<char> = iterator.next().unwrap().chars().collect();

    if t.len() > s_.len() {
        return "UNRESTORABLE".to_string();
    }

    let mut candidates: Vec<String> = Vec::new();
    for i in 0..(s_.len() - t.len() + 1) {
        let mut can_replace = true;
        let mut candidate = s_.clone();
        for j in 0..t.len() {
            if candidate[i + j] != '?' && candidate[i + j] != t[j] {
                can_replace = false;
                break;
            } else {
                candidate[i + j] = t[j];
            }
        }
        if can_replace {
            let s: String = candidate.into_iter().collect::<String>();
            candidates.push(s.replace('?', "a"));
        }
    }

    if candidates.is_empty() {
        return "UNRESTORABLE".to_string();
    }

    candidates.sort();

    return candidates[0].clone();
}

#[test]
fn test() {
    let cases: Vec<(&str, &str)> = vec![
        (
            r"?tc????
coder",
            "atcoder"
        ),
        (
            r"??p??d??
abc",
            "UNRESTORABLE"
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
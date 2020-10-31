use std::io::Read;

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

    if s.len() < t.len() {
        return "UNRESTORABLE".to_string();
    }

    let mut result = Vec::new();
    for i in 0..=(s.len() - t.len()) {
        let mut is_ok = true;
        let mut sc = s.clone();
        for j in 0..t.len() {
            if s[i + j] == '?' || s[i + j] == t[j] {
                sc[i + j] = t[j];
                continue;
            }
            is_ok = false;
            break;
        }
        if is_ok {
            for i in 0..sc.len() {
                if sc[i] == '?' {
                    sc[i] = 'a';
                }
            }
            result.push(sc.iter().collect::<String>());
        }
    }

    if result.is_empty() {
        "UNRESTORABLE".to_string()
    } else {
        result.iter().min().unwrap().to_string()
    }
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
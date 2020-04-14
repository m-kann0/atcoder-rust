use std::io::Read;
use std::collections::BTreeSet;

fn main() {
    let mut buf = String::new();
    std::io::stdin().read_to_string(&mut buf).unwrap();

    let answer = solve(&buf);

    println!("{}", answer);
}

fn solve(input: &str) -> String {
    let iterator = input.split_whitespace();

    let mut users: BTreeSet<String> = BTreeSet::new();
    for token in iterator {
        let mut token = token.to_string();
        if !token.contains('@') {
            continue;
        }
        while token.contains("@@") {
            token = token.replace("@@", "@");
        }
        token = token.replace("@", " @");
        for s in token.split_whitespace() {
            if s.starts_with("@") && s.len() >= 2 {
                users.insert(s.replace("@", ""));
            }
        }
    }

    let mut result = String::new();
    for user in users {
        result.push_str(&format!("{}\n", user));
    }
    return result.trim().to_string();
}

#[test]
fn test() {
    let cases: Vec<(&str, &str)> = vec![
        (
            r"@codeformula why is this contest so easy",
            "codeformula"
        ),
        (
            r"myon @@c @a @aba@a @@bb bbb @@@@@ @a test  @ space  aaa test @a@a  test@takoyaki",
            "a
aba
bb
c
takoyaki"
        ),
        (
            r"no atmark",
            ""
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
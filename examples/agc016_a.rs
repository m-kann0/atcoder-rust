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

    let mut set_list: Vec<HashSet<char>> = vec![HashSet::new(); s.len()];
    for (i, c) in s.chars().enumerate() {
        set_list[i].insert(c);
    }

    let mut done = false;
    let mut count = 0;
    while !done {
        for c in 'a' as u8..'z' as u8 + 1 {
            let c = c as char;
            if set_list.iter().all(|set| set.contains(&c)) {
                done = true;
                break;
            }
        }
        if !done {
            for i in 0..(set_list.len() - 1) {
                let next = set_list[i + 1].clone();
                set_list[i].extend(next);
            }
            set_list.pop();
            count += 1;
        }
    }

    return count.to_string();
}

#[test]
fn test() {
    let cases: Vec<(&str, &str)> = vec![
        (
            r"serval",
            "3"
        ),
        (
            r"jackal",
            "2"
        ),
        (
            r"zzz",
            "0"
        ),
        (
            r"whbrjpjyhsrywlqjxdbrbaomnw", // 26
            "8"
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
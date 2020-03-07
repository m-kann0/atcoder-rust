use std::io::Read;
use std::collections::VecDeque;

fn main() {
    let mut buf = String::new();
    std::io::stdin().read_to_string(&mut buf).unwrap();

    let answer = solve(&buf);

    println!("{}", answer);
}

fn solve(input: &str) -> String {
    let mut iterator = input.split_whitespace();

    let s: String = iterator.next().unwrap().to_string();
    let q: usize = iterator.next().unwrap().parse().unwrap();

    let mut vec: VecDeque<char> = s.chars().collect::<VecDeque<char>>();

    let mut is_reverse = false;
    for _i in 0..q {
        let t = iterator.next().unwrap();
        if t == "1" {
            is_reverse = !is_reverse;
        } else {
            let f = iterator.next().unwrap();
            let c = iterator.next().unwrap().chars().next().unwrap();
            if f == "1" {
                if is_reverse {
                    vec.push_back(c);
                } else {
                    vec.push_front(c);
                }
            } else {
                if is_reverse {
                    vec.push_front(c);
                } else {
                    vec.push_back(c);
                }
            }
        }
    }

    if is_reverse {
        vec.into_iter().rev().collect::<String>()
    } else {
        vec.into_iter().collect::<String>()
    }
}

#[test]
fn test() {
    let cases: Vec<(&str, &str)> = vec![
        (
            r"a
4
2 1 p
1
2 2 c
1",
            "cpa"
        ),
        (
            r"a
6
2 2 a
2 1 b
1
2 2 c
1
1",
            "aabc"
        ),
        (
            r"y
1
2 1 x",
            "xy"
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
use std::io::Read;
use std::collections::HashMap;

fn main() {
    let mut buf = String::new();
    std::io::stdin().read_to_string(&mut buf).unwrap();

    let answer = solve(&buf);

    println!("{}", answer);
}

fn solve(input: &str) -> String {
    let mut iterator = input.split_whitespace();

    let h: usize = iterator.next().unwrap().parse().unwrap();
    let w: usize = iterator.next().unwrap().parse().unwrap();
    let mut counts: HashMap<char, usize> = HashMap::new();
    for _ in 0..h {
        let line: Vec<char> = iterator.next().unwrap().chars().collect();
        for c in line {
            *counts.entry(c).or_insert(0) += 1;
        }
    }

    if h % 2 == 0 && w % 2 == 0 {
        let mut ok = true;
        for (_, count) in counts {
            if count % 4 != 0 {
                ok = false;
                break;
            }
        }
        return if ok {
            "Yes".to_string()
        } else {
            "No".to_string()
        };
    }

    if h % 2 == 0 && w % 2 == 1 || h % 2 == 1 && w % 2 == 0 {
        let even = if h % 2 == 0 { h } else { w };
        let mut mod4_2: usize = 0;
        for key in counts.clone().keys() {
            if counts.get(key).unwrap() % 4 == 2 {
                mod4_2 += 1;
                *counts.entry(*key).or_insert(0) -= 2;
            }
        }
        if mod4_2 > even / 2 {
            return "No".to_string();
        }
        let mut ok = true;
        for (_, count) in counts {
            if count % 4 != 0 {
                ok = false;
                break;
            }
        }
        return if ok {
            "Yes".to_string()
        } else {
            "No".to_string()
        };
    }

    for key in counts.clone().keys() {
        if counts.get(key).unwrap() % 2 == 1 {
            *counts.entry(*key).or_insert(0) -= 1;
            break;
        }
    }

    let mut mod4_2: usize = 0;
    for key in counts.clone().keys() {
        if counts.get(key).unwrap() % 4 == 2 {
            mod4_2 += 1;
            *counts.entry(*key).or_insert(0) -= 2;
        }
    }
    if mod4_2 > h / 2 + w / 2 {
        return "No".to_string();
    }
    let mut ok = true;
    for (_, count) in counts {
        if count % 4 != 0 {
            ok = false;
            break;
        }
    }
    return if ok {
        "Yes".to_string()
    } else {
        "No".to_string()
    };
}

#[test]
fn test() {
    let cases: Vec<(&str, &str)> = vec![
        (
            r"3 4
aabb
aabb
aacc",
            "Yes"
        ),
        (
            r"2 2
aa
bb",
            "No"
        ),
        (
            r"5 1
t
w
e
e
t",
            "Yes"
        ),
        (
            r"2 5
abxba
abyba",
            "No"
        ),
        (
            r"1 1
z",
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
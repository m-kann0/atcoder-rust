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

    let n: usize = iterator.next().unwrap().parse().unwrap();

    let mut counts: HashMap<usize, usize> = HashMap::new();
    for _ in 0..n {
        let ai: usize = iterator.next().unwrap().parse().unwrap();
        *counts.entry(ai).or_insert(0) += 1;
    }

    let mut keys: Vec<&usize> = counts.keys().collect::<Vec<_>>();
    keys.sort();

    let mut edge1: usize = 0;
    for &&key in keys.iter().rev() {
        if edge1 == 0 {
            if counts[&key] >= 4 {
                return format!("{}", key * key);
            } else if counts[&key] >= 2 {
                edge1 = key;
            }
        } else {
            if counts[&key] >= 2 {
                return format!("{}", edge1 * key);
            }
        }
    }

    return "0".into();
}

#[test]
fn test() {
    let cases: Vec<(&str, &str)> = vec![
        (
            r"6
3 1 2 4 2 1",
            "2"
        ),
        (
            r"4
1 2 3 4",
            "0"
        ),
        (
            r"10
3 3 3 3 4 4 4 5 5 5",
            "20"
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
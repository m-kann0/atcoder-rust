use std::io::Read;
use std::collections::HashMap;
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
    let k: usize = iterator.next().unwrap().parse().unwrap();

    let mut map: HashMap<usize, usize> = HashMap::with_capacity(n);
    for _ in 0..n {
        let a: usize = iterator.next().unwrap().parse().unwrap();
        *map.entry(a).or_insert(0) += 1;
    }

    let mut values: Vec<usize> = map.values().map(|u| u.clone()).collect();
    values.sort();
    for _ in 0..min(k, map.len()) {
        values.pop().unwrap();
    }

    let ans: usize = values.iter().sum();

    return ans.to_string();
}

#[test]
fn test() {
    let cases: Vec<(&str, &str)> = vec![
        (
            r"5 2
1 1 2 2 5",
            "1"
        ),
        (
            r"4 4
1 1 2 2",
            "0"
        ),
        (
            r"10 3
5 1 3 2 4 1 1 2 3 4",
            "3"
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
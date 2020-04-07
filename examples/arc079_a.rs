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

    let n: usize = iterator.next().unwrap().parse().unwrap();
    let m: usize = iterator.next().unwrap().parse().unwrap();

    let mut from_first: HashSet<usize> = HashSet::new();
    let mut to_last: HashSet<usize> = HashSet::new();
    for _ in 0..m {
        let a: usize = iterator.next().unwrap().parse().unwrap();
        let b: usize = iterator.next().unwrap().parse().unwrap();
        if a == 1 {
            from_first.insert(b);
        }
        if b == n {
            to_last.insert(a);
        }
    }

    if from_first.intersection(&to_last).count() >= 1 {
        return String::from("POSSIBLE")
    }

    return String::from("IMPOSSIBLE");
}

#[test]
fn test() {
    let cases: Vec<(&str, &str)> = vec![
        (
            r"3 2
1 2
2 3",
            "POSSIBLE"
        ),
        (
            r"4 3
1 2
2 3
3 4",
            "IMPOSSIBLE"
        ),
        (
            r"100000 1
1 99999",
            "IMPOSSIBLE"
        ),
        (
            r"5 5
1 3
4 5
2 3
2 4
1 4",
            "POSSIBLE"
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
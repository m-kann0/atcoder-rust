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

    let mut set: HashSet<usize> = HashSet::with_capacity(n);

    for _ in 0..n {
        let a: usize = iterator.next().unwrap().parse().unwrap();
        if set.contains(&a) {
            set.remove(&a);
        } else {
            set.insert(a);
        }
    }

    return set.len().to_string();
}

#[test]
fn test() {
    let cases: Vec<(&str, &str)> = vec![
        (
            r"3
6
2
6",
            "1"
        ),
        (
            r"4
2
5
5
2",
            "0"
        ),
        (
            r"6
12
22
16
22
18
12",
            "2"
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
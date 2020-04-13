use std::io::Read;
use std::collections::BinaryHeap;

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
    if m > n {
        return "NO".to_string();
    }

    let mut a: BinaryHeap<usize> = BinaryHeap::new();
    for _ in 0..n {
        a.push(iterator.next().unwrap().parse().unwrap());
    }

    let mut b: BinaryHeap<usize> = BinaryHeap::new();
    for _ in 0..m {
        b.push(iterator.next().unwrap().parse().unwrap());
    }

    while !b.is_empty() {
        if b.pop().unwrap() > a.pop().unwrap() {
            return "NO".to_string();
        }
    }

    return "YES".to_string();
}

#[test]
fn test() {
    let cases: Vec<(&str, &str)> = vec![
        (
            r"3 2
2 2 3
3 1",
            "YES"
        ),
        (
            r"3 2
2 2 3
3 3",
            "NO"
        ),
        (
            r"3 4
10 10 10
1 1 1 1",
            "NO"
        ),
        (
            r"10 10
10 9 8 7 6 5 4 3 2 1
10 9 8 7 6 5 4 3 2 1",
            "YES"
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
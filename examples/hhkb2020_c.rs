use std::io::Read;
use std::collections::{BinaryHeap, HashSet};
use std::cmp::Reverse;

fn main() {
    let mut buf = String::new();
    std::io::stdin().read_to_string(&mut buf).unwrap();

    let answer = solve(&buf);

    println!("{}", answer);
}

fn solve(input: &str) -> String {
    let mut iterator = input.split_whitespace();

    let n: usize = iterator.next().unwrap().parse().unwrap();
    let p: Vec<usize> = (0..n).map(|_| iterator.next().unwrap().parse().unwrap()).collect();

    let mut pq: BinaryHeap<Reverse<usize>> = BinaryHeap::new();
    for i in 0..=200_001 {
        pq.push(Reverse(i));
    }

    let mut result = String::new();
    let mut used: HashSet<usize> = HashSet::new();
    for i in 0..n {
        used.insert(p[i]);
        loop {
            let m = pq.pop().unwrap().0;
            if !used.contains(&m) {
                result.push_str(&format!("{}\n", m));
                pq.push(Reverse(m));
                break;
            }
        }
    }
    result.trim().to_string()
}

#[test]
fn test() {
    let cases: Vec<(&str, &str)> = vec![
        (
            r"4
1 1 0 2",
            "0
0
2
3"
        ),
        (
            r"10
5 4 3 2 1 0 7 7 6 6",
            "0
0
0
0
0
6
6
6
8
8"
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
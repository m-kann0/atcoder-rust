use std::io::Read;
use std::collections::BinaryHeap;
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
    let mut graph: Vec<Vec<usize>> = vec![Vec::new(); n];
    for _ in 0..(n - 1) {
        let ai: usize = iterator.next().unwrap().parse().unwrap();
        let bi: usize = iterator.next().unwrap().parse().unwrap();
        let ai = ai - 1;
        let bi = bi - 1;
        graph[ai].push(bi);
        graph[bi].push(ai);
    }

    let mut result = String::new();
    let mut heap: BinaryHeap<Reverse<usize>> = BinaryHeap::new();
    let mut visited: Vec<bool> = vec![false; n];
    heap.push(Reverse(0));
    visited[0] = true;
    while let Some(Reverse(u)) = heap.pop() {
        result.push_str(&format!("{} ", u + 1));
        for &v in &graph[u] {
            if visited[v] {
                continue;
            }
            heap.push(Reverse(v));
            visited[v] = true;
        }
    }
    result.trim().to_string()
}

#[test]
fn test() {
    let cases: Vec<(&str, &str)> = vec![
        (
            r"4
1 3
1 4
2 3",
            "1 3 2 4"
        ),
        (
            r"6
1 2
2 3
2 6
6 4
1 5",
            "1 2 3 5 6 4"
        ),
        (
            r"7
1 5
5 2
5 3
5 7
5 6
6 4",
            "1 5 2 3 6 4 7"
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
#![allow(non_snake_case)]

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

    let n: usize = iterator.next().unwrap().parse().unwrap();
    let m: usize = iterator.next().unwrap().parse().unwrap();
    let mut graph = vec![Vec::new(); n];
    for _ in 0..m {
        let u: usize = iterator.next().unwrap().parse().unwrap();
        let v: usize = iterator.next().unwrap().parse().unwrap();
        let c: usize = iterator.next().unwrap().parse().unwrap();
        graph[u - 1].push((v - 1, c));
        graph[v - 1].push((u - 1, c));
    }

    const INF: usize = 1_000_000;
    let mut numbers = vec![INF; n];
    let mut q = VecDeque::new();
    q.push_back((0, 0, INF));
    while let Some((u, from, from_c)) = q.pop_front() {
        if numbers[from] != from_c {
            numbers[u] = from_c;
        }
        if numbers[u] == INF {
            let mut number = INF;
            for &(v, c) in &graph[u] {
                if numbers[v] != INF {
                    continue;
                }
                if c != numbers[from] {
                    number = c;
                    break;
                }
            }
            if number == INF {
                number = if numbers[from] < n {
                    numbers[from] + 1
                } else {
                    1
                };
            }
            numbers[u] = number;
        }
        for &(v, c) in &graph[u] {
            if numbers[v] != INF {
                continue;
            }
            q.push_back((v, u, c));
        }
        // eprintln!("numbers = {:?}", numbers);
    }
    let mut result = String::new();
    for i in 0..n {
        if numbers[i] == INF {
            return "No".to_string();
        }
        result.push_str(&format!("{}\n", numbers[i]));
    }
    result.trim().to_string()
}

#[test]
fn test() {
    let cases: Vec<(&str, &str)> = vec![
        (
            r"3 4
1 2 1
2 3 2
3 1 3
1 3 1",
            "1
2
1"
        ),
        (
            r"5 4
1 2 1
2 3 2
2 4 4
2 5 2",
            "1
2
1"
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

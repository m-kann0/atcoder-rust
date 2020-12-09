#![allow(non_snake_case)]

use std::io::Read;

fn main() {
    let mut buf = String::new();
    std::io::stdin().read_to_string(&mut buf).unwrap();

    let answer = solve(&buf);

    println!("{}", answer);
}

fn solve(input: &str) -> String {
    let mut iterator = input.split_whitespace();

    let n: usize = iterator.next().unwrap().parse().unwrap();
    let q: usize = iterator.next().unwrap().parse().unwrap();
    let mut graph = vec![vec![]; n];
    for _ in 0..(n - 1) {
        let a: usize = iterator.next().unwrap().parse().unwrap();
        let b: usize = iterator.next().unwrap().parse().unwrap();
        graph[a - 1].push(b - 1);
        graph[b - 1].push(a - 1);
    }
    let mut counter = vec![0; n];
    for _ in 0..q {
        let p: usize = iterator.next().unwrap().parse().unwrap();
        let x: usize = iterator.next().unwrap().parse().unwrap();
        counter[p - 1] += x;
    }

    const INF: usize = 1_000_000_000;
    dfs(0, INF, &mut counter, &graph);

    let mut result = String::new();
    for i in 0..n {
        result.push_str(&format!("{} ", counter[i]));
    }
    result
}

fn dfs(node: usize, parent: usize, counter: &mut Vec<usize>, graph: &Vec<Vec<usize>>) {
    for &next_node in &graph[node] {
        if next_node == parent {
            continue;
        }
        counter[next_node] += counter[node];
        dfs(next_node, node, counter, graph);
    }
}

#[test]
fn test() {
    let cases: Vec<(&str, &str)> = vec![
        (
            r"4 3
1 2
2 3
2 4
2 10
1 100
3 1",
            "100 110 111 110"
        ),
        (
            r"6 2
1 2
1 3
2 4
3 6
2 5
1 10
1 10",
            "20 20 20 20 20 20"
        ),
    ];

    let mut all_ok = true;
    for (i, case) in cases.iter().enumerate() {
        print!("case {} : ", i);

        let expected = case.1;
        let actual = solve(case.0);

        if expected.trim() == actual.trim() {
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
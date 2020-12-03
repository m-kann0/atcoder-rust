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
    let m: usize = iterator.next().unwrap().parse().unwrap();
    let mut graph: Vec<Vec<(usize, usize)>> = vec![Vec::new(); n];
    for _ in 0..m {
        let l: usize = iterator.next().unwrap().parse().unwrap();
        let r: usize = iterator.next().unwrap().parse().unwrap();
        let d: usize = iterator.next().unwrap().parse().unwrap();
        graph[l - 1].push((r - 1, d));
    }

    if !graph_tool::is_dag(&graph) {
        return "No".to_string();
    }

    let sorted = graph_tool::topological_sort(&graph);
    let mut position = vec![None; n];
    for u in sorted {
        if position[u] == None {
            position[u] = Some(0);
        }
        let p1 = position[u].unwrap();
        for &(v, d) in &graph[u] {
            if let Some(p2) = position[v] {
                if p2 != p1 + d {
                    return "No".to_string();
                }
            }
            position[v] = Some(p1 + d);
        }
    }
    "Yes".to_string()
}

mod graph_tool {
    pub fn is_dag(graph: &Vec<Vec<(usize, usize)>>) -> bool {
        fn rec(
            graph: &Vec<Vec<(usize, usize)>>,
            current: usize,
            from: usize,
            seen: &mut Vec<bool>,
            finished: &mut Vec<bool>
        ) -> bool {
            seen[current] = true;
            for &(next, _) in &graph[current] {
                if next == from {
                    return false;
                }
                if finished[next] {
                    continue;
                }
                // eprintln!("next = {:?}", next);
                // eprintln!("seen = {:?}", seen);
                // eprintln!("finished = {:?}", finished);
                if seen[next] && !finished[next] {
                    return false;
                }

                let result = rec(graph, next, current, seen, finished);
                if !result {
                    return false;
                }
            }
            finished[current] = true;
            true
        }

        let n = graph.len();
        let INF = n + 10;
        let mut seen = vec![false; n];
        let mut finished = vec![false; n];
        for i in 0..n {
            if finished[i] {
                continue;
            }
            let result = rec(graph, i, INF, &mut seen, &mut finished);
            if !result {
                return false;
            }
        }
        true
    }

    pub fn topological_sort(graph: &Vec<Vec<(usize, usize)>>) -> Vec<usize> {
        fn rec(
            graph: &Vec<Vec<(usize, usize)>>,
            v: usize,
            seen: &mut Vec<bool>,
            result: &mut Vec<usize>
        ) {
            seen[v] = true;
            for &(next_v, _) in &graph[v] {
                if seen[next_v] {
                    continue;
                }
                rec(graph, next_v, seen, result);
            }
            result.push(v);
        }

        assert!(is_dag(graph), "graph is not DAG");

        let mut seen = vec![false; graph.len()];
        let mut result = Vec::new();
        for v in 0..graph.len() {
            if seen[v] {
                continue;
            }
            rec(graph, v, &mut seen, &mut result);
        }
        result.reverse();
        result
    }
}

#[test]
fn test() {
    let cases: Vec<(&str, &str)> = vec![
        (
            r"3 3
1 2 1
2 3 1
1 3 2",
            "Yes"
        ),
        (
            r"3 3
1 2 1
2 3 1
1 3 5",
            "No"
        ),
        (
            r"4 3
2 1 1
2 3 5
3 4 2",
            "Yes"
        ),
        (
            r"10 3
8 7 100
7 9 100
9 8 100",
            "No"
        ),
        (
            r"100 0",
            "Yes"
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
use std::io::Read;
use std::collections::{HashSet, VecDeque, HashMap};

fn main() {
    let mut buf = String::new();
    std::io::stdin().read_to_string(&mut buf).unwrap();

    let answer = solve(&buf);

    println!("{}", answer);
}

fn solve(input: &str) -> String {
    let mut iterator = input.split_whitespace();

    let n: usize = iterator.next().unwrap().parse().unwrap();
    let edges: Vec<(usize, usize)> =
        (0..(n - 1))
            .map(|_| {
                let ai: usize = iterator.next().unwrap().parse().unwrap();
                let bi: usize = iterator.next().unwrap().parse().unwrap();
                (ai - 1, bi - 1)
            })
            .collect();

    let mut graph: Vec<Vec<usize>> = vec![vec![]; n];
    for &edge in &edges {
        graph[edge.0].push(edge.1);
        graph[edge.1].push(edge.0);
    }
    let k: usize = graph.iter().map(|it| it.len()).max().unwrap();

    let mut colors: HashMap<(usize, usize), usize> = HashMap::new();
    let mut used: Vec<HashSet<usize>> = vec![HashSet::new(); n];
    let mut visited: Vec<bool> = vec![false; n];
    let mut q = VecDeque::new();
    q.push_back(0_usize);
    visited[0] = true;
    while let Some(u) = q.pop_front() {
        let mut color = 1;
        for &v in &graph[u] {
            if visited[v] {
                continue;
            }
            if used[u].contains(&color) {
                color += 1;
            }
            if u < v {
                colors.insert((u, v), color);
            } else {
                colors.insert((v, u), color);
            }
            used[u].insert(color);
            used[v].insert(color);
            visited[v] = true;
            q.push_back(v);
            color += 1;
        }
    }

    let mut result = String::new();
    result.push_str(&format!("{}\n", k));
    for &edge in &edges {
        let color = colors.get(&edge).unwrap();
        result.push_str(&format!("{}\n", color));
    }
    result.trim().to_string()
}

#[test]
fn test() {
    let cases: Vec<(&str, &str)> = vec![
        (
            r"3
1 2
2 3",
            "2
1
2"
        ),
        (
            r"8
1 2
2 3
2 4
2 5
4 7
5 6
6 8",
            "4
1
2
3
4
1
1
2"
        ),
        (
            r"6
1 2
1 3
1 4
1 5
1 6",
            "5
1
2
3
4
5"
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
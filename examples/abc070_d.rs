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
    let mut graph: Vec<Vec<(usize, usize)>> = vec![vec![]; n];
    for _ in 0..(n - 1) {
        let ai: usize = iterator.next().unwrap().parse().unwrap();
        let bi: usize = iterator.next().unwrap().parse().unwrap();
        let ci: usize = iterator.next().unwrap().parse().unwrap();
        let ai = ai - 1;
        let bi = bi - 1;
        graph[ai].push((bi, ci));
        graph[bi].push((ai, ci));
    }

    let q: usize = iterator.next().unwrap().parse().unwrap();
    let k: usize = iterator.next().unwrap().parse().unwrap();
    let k = k - 1;

    const INF: usize = std::usize::MAX;
    let mut costs: Vec<usize> = vec![INF; n];
    costs[k] = 0;
    let mut queue = VecDeque::new();
    queue.push_back((k, 0_usize));
    while let Some((u, cost)) = queue.pop_front() {
        for &(v, c) in &graph[u] {
            if costs[v] != INF {
                continue;
            }
            costs[v] = cost + c;
            queue.push_back((v, costs[v]));
        }
    }

    let mut result = String::new();
    for _ in 0..q {
        let x1: usize = iterator.next().unwrap().parse().unwrap();
        let y1: usize = iterator.next().unwrap().parse().unwrap();
        let x1 = x1 - 1;
        let y1 = y1 - 1;
        result.push_str(&format!("{}\n", costs[x1] + costs[y1]));
    }
    result.trim().to_string()
}

#[test]
fn test() {
    let cases: Vec<(&str, &str)> = vec![
        (
            r"5
1 2 1
1 3 1
2 4 1
3 5 1
3 1
2 4
2 3
4 5",
            "3
2
4"
        ),
        (
            r"7
1 2 1
1 3 3
1 4 5
1 5 7
1 6 9
1 7 11
3 2
1 3
4 5
6 7",
            "5
14
22"
        ),
        (
            r"10
1 2 1000000000
2 3 1000000000
3 4 1000000000
4 5 1000000000
5 6 1000000000
6 7 1000000000
7 8 1000000000
8 9 1000000000
9 10 1000000000
1 1
9 10",
            "17000000000"
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
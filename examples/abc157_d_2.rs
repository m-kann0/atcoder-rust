use std::io::Read;
use std::collections::{HashMap, VecDeque};
use itertools::Itertools;

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
    let k: usize = iterator.next().unwrap().parse().unwrap();
    let mut graph = vec![Vec::new(); n];
    for _ in 0..m {
        let ai: usize = iterator.next().unwrap().parse().unwrap();
        let bi: usize = iterator.next().unwrap().parse().unwrap();
        graph[ai - 1].push(bi - 1);
        graph[bi - 1].push(ai - 1);
    }
    let mut blocks: Vec<(usize, usize)> = Vec::new();
    for _ in 0..k {
        let ci: usize = iterator.next().unwrap().parse().unwrap();
        let di: usize = iterator.next().unwrap().parse().unwrap();
        blocks.push((ci - 1, di - 1));
    }

    let mut colors: Vec<Option<usize>> = vec![None; n];
    let mut color: usize = 0;
    let mut counts: HashMap<usize, usize> = HashMap::new();
    for i in 0..n {
        if colors[i].is_some() {
            continue;
        }
        let mut q: VecDeque<usize> = VecDeque::new();
        q.push_back(i);
        colors[i] = Some(color);
        *counts.entry(color).or_insert(0) += 1;
        while let Some(u) = q.pop_front() {
            for &v in &graph[u] {
                if colors[v].is_some() {
                    continue;
                }
                q.push_back(v);
                colors[v] = Some(color);
                *counts.entry(color).or_insert(0) += 1;
            }
        }
        color += 1;
    }

    let mut result = vec![0; n];
    for i in 0..n {
        result[i] = *counts.get(&colors[i].unwrap()).unwrap() - graph[i].len() - 1;
    }

    for b in blocks {
        if colors[b.0] == colors[b.1] {
            result[b.0] -= 1;
            result[b.1] -= 1;
        }
    }

    result.iter().map(|it| it.to_string()).join(" ")
}

#[test]
fn test() {
    let cases: Vec<(&str, &str)> = vec![
        (
            r"4 4 1
2 1
1 3
3 2
3 4
4 1",
            "0 1 0 1"
        ),
        (
            r"5 10 0
1 2
1 3
1 4
1 5
3 2
2 4
2 5
4 3
5 3
4 5",
            "0 0 0 0 0"
        ),
        (
            r"10 9 3
10 1
6 7
8 2
2 5
8 4
7 3
10 9
6 4
5 8
2 6
7 5
3 1",
            "1 3 5 4 3 3 3 3 1 0"
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
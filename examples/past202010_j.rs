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
    let m: usize = iterator.next().unwrap().parse().unwrap();
    let xab: usize = iterator.next().unwrap().parse().unwrap();
    let xac: usize = iterator.next().unwrap().parse().unwrap();
    let xbc: usize = iterator.next().unwrap().parse().unwrap();
    let s: Vec<char> = iterator.next().unwrap().chars().collect();
    let mut graph: Vec<Vec<(usize, usize)>> = vec![Vec::new(); n + 6];
    for _ in 0..m {
        let a: usize = iterator.next().unwrap().parse().unwrap();
        let b: usize = iterator.next().unwrap().parse().unwrap();
        let c: usize = iterator.next().unwrap().parse().unwrap();
        let a = a - 1;
        let b = b - 1;
        graph[a].push((b, c));
        graph[b].push((a, c));
    }

    let a_in = n;
    let a_out = n + 1;
    let b_in = n + 2;
    let b_out = n + 3;
    let c_in = n + 4;
    let c_out = n + 5;
    graph[a_in].push((b_out, xab));
    graph[a_in].push((c_out, xac));
    graph[b_in].push((a_out, xab));
    graph[b_in].push((c_out, xbc));
    graph[c_in].push((a_out, xac));
    graph[c_in].push((b_out, xbc));
    for i in 0..n {
        match s[i] {
            'A' => {
                graph[i].push((a_in, 0));
                graph[a_out].push((i, 0));
            },
            'B' => {
                graph[i].push((b_in, 0));
                graph[b_out].push((i, 0));
            },
            'C' => {
                graph[i].push((c_in, 0));
                graph[c_out].push((i, 0));
            },
            _ => unreachable!()
        }
    }

    const INF: usize = std::usize::MAX;
    let mut q: BinaryHeap<Reverse<(usize, usize)>> = BinaryHeap::new();
    let mut d: Vec<usize> = vec![INF; n + 6];
    d[0] = 0;
    q.push(Reverse((0, 0)));
    while let Some(Reverse((dv, v))) = q.pop() {
        if d[v] < dv {
            continue;
        }
        for &(to, cost) in &graph[v] {
            if d[to] > d[v] + cost {
                d[to] = d[v] + cost;
                q.push(Reverse((d[to], to)));
            }
        }
    }
    d[n - 1].to_string()
}

#[test]
fn test() {
    let cases: Vec<(&str, &str)> = vec![
        (
            r"3 2
10 10 10
ABA
1 2 15
2 3 5",
            "15"
        ),
        (
            r"3 2
10 1000000000 10
ABC
1 2 1000000000
2 3 1000000000",
            "20"
        ),
        (
            r"5 6
5 10 15
ABCBC
5 4 4
3 5 2
1 3 7
3 4 1
4 2 1
2 3 3",
            "8"
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
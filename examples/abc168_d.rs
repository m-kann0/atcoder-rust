use std::io::Read;
use std::cmp::Reverse;
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

    let mut graph: Vec<Vec<usize>> = vec![Vec::new(); n];
    for _ in 0..m {
        let a: usize = iterator.next().unwrap().parse().unwrap();
        let b: usize = iterator.next().unwrap().parse().unwrap();
        graph[a - 1].push(b - 1);
        graph[b - 1].push(a - 1);
    }

    const INF: usize = std::usize::MAX;

    let mut ans: Vec<usize> = vec![0; n];

    let mut d: Vec<usize> = vec![INF; n];
    let mut color: Vec<Color> = vec![Color::WHITE; n];
    let mut pq: BinaryHeap<Reverse<(usize, usize)>> = BinaryHeap::new();

    d[0] = 0;
    pq.push(Reverse((0, 0)));
    color[0] = Color::GRAY;

    while !pq.is_empty() {
        let f = pq.pop().unwrap().0;

        let u = f.1;
        color[u] = Color::BLACK;

        if d[u] < f.0 {
            continue;
        }

        for j in 0..graph[u].len() {
            let v = graph[u][j];
            if color[v] == Color::BLACK {
                continue;
            }
            if d[v] > d[u] + 1 {
                d[v] = d[u] + 1;
                ans[v] = u;
                pq.push(Reverse((d[v], v)));
                color[v] = Color::GRAY;
            }
        }
    }

    let mut result = String::from("Yes\n");
    for i in 1..n {
        result.push_str(&format!("{}\n", ans[i] + 1));
    }
    return result.trim().to_string();
}

#[derive(Clone, PartialEq)]
enum Color { WHITE, GRAY, BLACK }

#[test]
fn test() {
    let cases: Vec<(&str, &str)> = vec![
        (
            r"4 4
1 2
2 3
3 4
4 2",
            "Yes
1
2
2"
        ),
        (
            r"6 9
3 4
6 1
2 4
5 3
4 6
1 5
6 2
4 5
5 6",
            "Yes
6
5
5
1
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
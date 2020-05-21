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
    let q: usize = iterator.next().unwrap().parse().unwrap();
    let mut tree: Vec<Vec<usize>> = vec![Vec::new(); n];
    for _ in 0..(n - 1) {
        let ai: usize = iterator.next().unwrap().parse().unwrap();
        let bi: usize = iterator.next().unwrap().parse().unwrap();
        tree[ai - 1].push(bi - 1);
        tree[bi - 1].push(ai - 1);
    }
    let mut p: Vec<usize> = vec![0; n];
    for _ in 0..q {
        let pi: usize = iterator.next().unwrap().parse().unwrap();
        let xi: usize = iterator.next().unwrap().parse().unwrap();
        p[pi - 1] += xi;
    }

    let mut result: Vec<usize> = vec![0; n];
    let mut visited: Vec<bool> = vec![false; n];
    let mut queue: VecDeque<usize> = VecDeque::new();
    visited[0] = true;
    queue.push_back(0);
    while !queue.is_empty() {
        let u: usize = queue.pop_front().unwrap();
        result[u] += p[u];
        for &v in &tree[u] {
            if visited[v] {
                continue;
            }
            visited[v] = true;
            result[v] += result[u];
            queue.push_back(v);
        }
    }
    return result.iter().map(|it| it.to_string()).collect::<Vec<String>>().join(" ");
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
        (
            r"3 3
1 3
2 3
2 10
3 100
1 1",
            "1 111 101"
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
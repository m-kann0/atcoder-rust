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
    let mut graph: Vec<Vec<usize>> = vec![Vec::new(); n];
    for _ in 0..(n - 1) {
        let ai: usize = iterator.next().unwrap().parse().unwrap();
        let bi: usize = iterator.next().unwrap().parse().unwrap();
        graph[ai - 1].push(bi - 1);
        graph[bi - 1].push(ai - 1);
    }
    let mut c: Vec<usize> = (0..n).map(|_| iterator.next().unwrap().parse().unwrap()).collect();
    c.sort();
    c.reverse();

    let mut m: usize = 0;
    for i in 1..n {
        m += c[i];
    }

    let mut i: usize = 0;
    let mut d: Vec<usize> = vec![0; n];
    let mut q: VecDeque<usize> = VecDeque::new();
    q.push_back(0);
    d[0] = c[i];
    i += 1;
    while let Some(u) = q.pop_front() {
        for &v in &graph[u] {
            if d[v] != 0 {
                continue;
            }
            d[v] = c[i];
            i += 1;
            q.push_back(v);
        }
    }

    let mut result = String::new();
    result.push_str(&format!("{}\n", m));
    for i in 0..n {
        result.push_str(&format!("{} ", d[i]));
    }
    result.trim().to_string()
}

#[test]
fn test() {
    let cases: Vec<(&str, &str)> = vec![
        (
            r"5
1 2
2 3
3 4
4 5
1 2 3 4 5",
            "10
1 2 3 4 5"
        ),
        (
            r"5
1 2
1 3
1 4
1 5
3141 59 26 53 59",
            "197
59 26 3141 59 53"
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
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
    let x: usize = iterator.next().unwrap().parse().unwrap();
    let x = x - 1;
    let h: Vec<usize> = (0..n).map(|_| iterator.next().unwrap().parse().unwrap()).collect();
    let mut graph: Vec<Vec<usize>> = vec![Vec::new(); n];
    for _ in 0..(n - 1) {
        let a: usize = iterator.next().unwrap().parse().unwrap();
        let b: usize = iterator.next().unwrap().parse().unwrap();
        graph[a - 1].push(b - 1);
        graph[b - 1].push(a - 1);
    }

    let result = dfs(&h, &graph, -1, x);
    result.1.to_string()
}

fn dfs(h: &Vec<usize>, graph: &Vec<Vec<usize>>, parent: isize, current: usize) -> (bool, usize) {
    let mut has_hoseki = h[current] > 0;
    let mut cost = 0;
    for &v in &graph[current] {
        if v as isize == parent {
            continue;
        }
        let result = dfs(h, graph, current as isize, v);
        if result.0 {
            has_hoseki = true;
            cost += result.1 + 2;
        }
    }
    (has_hoseki, cost)
}

#[test]
fn test() {
    let cases: Vec<(&str, &str)> = vec![
        (
            r"5 1
1 0 1 0 1
1 2
2 3
2 4
1 5",
            "6"
        ),
        (
            r"3 2
0 1 0
1 2
2 3",
            "0"
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

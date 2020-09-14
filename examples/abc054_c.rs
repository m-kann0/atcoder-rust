use std::io::Read;
use std::collections::HashSet;

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
        let ai: usize = iterator.next().unwrap().parse().unwrap();
        let bi: usize = iterator.next().unwrap().parse().unwrap();
        let ai = ai - 1;
        let bi = bi - 1;
        graph[ai].push(bi);
        graph[bi].push(ai);
    }

    let mut used: HashSet<usize> = HashSet::new();
    used.insert(0);
    let ans = dfs(&graph, &used, 0, n);
    ans.to_string()
}

fn dfs(graph: &Vec<Vec<usize>>, used: &HashSet<usize>, current: usize, n: usize) -> usize {
    if used.len() == n {
        return 1;
    }
    let mut ans = 0;
    for &v in graph[current].iter() {
        if used.contains(&v) {
            continue;
        }
        let mut used = used.clone();
        used.insert(v);
        ans += dfs(graph, &used, v, n);
    }
    ans
}

#[test]
fn test() {
    let cases: Vec<(&str, &str)> = vec![
        (
            r"3 3
1 2
1 3
2 3",
            "2"
        ),
        (
            r"7 7
1 3
2 7
3 4
4 5
4 6
5 6
6 7",
            "1"
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
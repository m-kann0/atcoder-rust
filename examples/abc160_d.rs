use std::io::Read;
use std::collections::{VecDeque, HashMap};

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
    let y: usize = iterator.next().unwrap().parse().unwrap();

    let mut map: Vec<Vec<bool>> = vec![vec![false; n]; n];
    for i in 0..(n - 1) {
        map[i][i + 1] = true;
        map[i + 1][i] = true;
    }
    map[x - 1][y - 1] = true;
    map[y - 1][x - 1] = true;

    let mut counts: HashMap<usize, usize> = HashMap::with_capacity(n);
    for i in 0..n {
        bfs(&map, i, &mut counts);
    }

    let mut ans = String::new();
    for k in 1..n {
        let default = 0;
        let count = counts.get(&k).unwrap_or(&default);
        ans.push_str(&format!("{}\n", count));
    }

    return ans.trim().to_string();
}

fn bfs(map: &Vec<Vec<bool>>, s: usize, counts: &mut HashMap<usize, usize>) {
    let n = map.len();
    let mut color: Vec<Color> = vec![Color::WHITE; n];
    let mut d: Vec<usize> = vec![std::usize::MAX; n];

    let mut q: VecDeque<usize> = VecDeque::new();
    color[s] = Color::GRAY;
    d[s] = 0;
    q.push_back(s);

    while !q.is_empty() {
        let u = q.pop_front().unwrap();
        for v in 0..n {
            if map[u][v] && color[v] == Color::WHITE {
                color[v] = Color::GRAY;
                d[v] = d[u] + 1;
                q.push_back(v);

                if v > s {
                    *counts.entry(d[v]).or_insert(0) += 1;
                }
            }
        }
        color[u] = Color::BLACK;
    }
}

#[derive(Clone, PartialEq)]
enum Color {
    WHITE, GRAY, BLACK,
}

#[test]
fn test() {
    let cases: Vec<(&str, &str)> = vec![
        (
            r"5 2 4",
            "5
4
1
0"
        ),
        (
            r"3 1 3",
            "3
0"
        ),
        (
            r"7 3 7",
            "7
8
4
2
0
0"
        ),
        (
            r"10 4 8",
            "10
12
10
8
4
1
0
0
0"
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
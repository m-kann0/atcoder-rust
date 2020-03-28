use std::io::Read;
use std::collections::VecDeque;

fn main() {
    let mut buf = String::new();
    std::io::stdin().read_to_string(&mut buf).unwrap();

    let answer = solve(&buf);

    println!("{}", answer);
}

const INF: isize = 100_001;

fn solve(input: &str) -> String {
    let mut iterator = input.split_whitespace();

    let n: isize = iterator.next().unwrap().parse().unwrap();
    let x: isize = iterator.next().unwrap().parse().unwrap();
    let y: isize = iterator.next().unwrap().parse().unwrap();

    let x = x - 1;
    let y = y - 1;

    let mut counts: Vec<usize> = vec![0; (n + 1) as usize];

    for sv in 0..n {
        let mut dist: Vec<isize> = vec![INF; n as usize];
        let mut q: VecDeque<isize> = VecDeque::new();

        dist[sv as usize] = 0;
        q.push_back(sv);

        while !q.is_empty() {
            let v = q.pop_front().unwrap();
            let d = dist[v as usize];
            if v - 1 >= 0 {
                if dist[(v - 1) as usize] == INF {
                    dist[(v - 1) as usize] = d + 1;
                    q.push_back(v - 1);
                }
            }
            if v + 1 < n {
                if dist[(v + 1) as usize] == INF {
                    dist[(v + 1) as usize] = d + 1;
                    q.push_back(v + 1);
                }
            }
            if v == x {
                if dist[y as usize] == INF {
                    dist[y as usize] = d + 1;
                    q.push_back(y);
                }
            }
            if v == y {
                if dist[x as usize] == INF {
                    dist[x as usize] = d + 1;
                    q.push_back(x);
                }
            }
        }

        for i in 0..n {
            counts[dist[i as usize] as usize] += 1;
        }
    }

    let mut ans = String::new();
    for i in 1..n {
        ans.push_str(&format!("{}\n", counts[i as usize] / 2))
    }
    return ans.trim().to_string();
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
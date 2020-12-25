#![allow(non_snake_case)]

use std::io::Read;
use std::cmp::min;

fn main() {
    let mut buf = String::new();
    std::io::stdin().read_to_string(&mut buf).unwrap();

    let answer = solve(&buf);

    println!("{}", answer);
}

fn solve(input: &str) -> String {
    let mut iterator = input.split_whitespace();

    let N: usize = iterator.next().unwrap().parse().unwrap();
    let M: usize = iterator.next().unwrap().parse().unwrap();
    let R: usize = iterator.next().unwrap().parse().unwrap();
    let r: Vec<usize> = (0..R).map(|_| iterator.next().unwrap().parse::<usize>().unwrap() - 1).collect();
    const INF: usize = 1_000_000_000_000;
    let mut d = vec![vec![INF; N]; N];
    for _ in 0..M {
        let ai: usize = iterator.next().unwrap().parse().unwrap();
        let bi: usize = iterator.next().unwrap().parse().unwrap();
        let ci: usize = iterator.next().unwrap().parse().unwrap();
        d[ai - 1][bi - 1] = ci;
        d[bi - 1][ai - 1] = ci;
    }

    for i in 0..N {
        d[i][i] = 0;
    }
    for k in 0..N {
        for i in 0..N {
            for j in 0..N {
                d[i][j] = min(d[i][j], d[i][k] + d[k][j]);
            }
        }
    }

    let mut ans: usize = INF;
    let perms = permutation::generate(R);
    for p in &perms {
        let mut now = 0;
        for i in 1..R {
            now += d[r[p[i]]][r[p[i - 1]]];
        }
        ans = min(ans, now);
    }
    ans.to_string()
}

mod permutation {
    pub fn generate(n: usize) -> Vec<Vec<usize>> {
        let mut used = vec![false; n];
        let mut perm = vec![0; n];
        let mut result = Vec::new();
        rec(0, n, &mut used, &mut perm, &mut result);
        result
    }

    fn rec(pos: usize, n: usize, used: &mut Vec<bool>, perm: &mut Vec<usize>, result: &mut Vec<Vec<usize>>) {
        if pos == n {
            result.push(perm.clone());
            return
        }

        for i in 0..n {
            if !used[i] {
                perm[pos] = i;
                used[i] = true;
                rec(pos + 1, n, used, perm, result);
                used[i] = false;
            }
        }
    }
}

#[test]
fn test() {
    let cases: Vec<(&str, &str)> = vec![
        (
            r"3 3 3
1 2 3
1 2 1
2 3 1
3 1 4",
            "2"
        ),
        (
            r"3 3 2
1 3
2 3 2
1 3 6
1 2 2",
            "4"
        ),
        (
            r"4 6 3
2 3 4
1 2 4
2 3 3
4 3 1
1 4 1
4 2 2
3 1 6",
            "3"
        ),
    ];

    let mut all_ok = true;
    for (i, case) in cases.iter().enumerate() {
        print!("case {} : ", i);

        let expected = case.1;
        let actual = solve(case.0);

        if expected.trim() == actual.trim() {
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
#![allow(non_snake_case)]

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
    let k: usize = iterator.next().unwrap().parse().unwrap();
    let mut t: Vec<Vec<usize>> = Vec::new();
    for _ in 0..n {
        let mut line = Vec::new();
        for _ in 0..n {
            let ti: usize = iterator.next().unwrap().parse().unwrap();
            line.push(ti);
        }
        t.push(line);
    }

    let mut ans: usize = 0;
    let perms = generate_permutation(n - 1);
    for perm in &perms {
        let mut now = 0;
        for i in 0..perm.len() {
            if i == 0 {
                now += t[0][perm[i] + 1];
            } else {
                now += t[perm[i - 1] + 1][perm[i] + 1];
            }
        }
        now += t[*perm.last().unwrap() + 1][0];
        // eprintln!("perm = {:?}", perm);
        // eprintln!("now = {:?}", now);
        if now == k {
            ans += 1;
        }
    }
    ans.to_string()
}

fn generate_permutation(n: usize) -> Vec<Vec<usize>> {
    let mut used = vec![false; n];
    let mut perm = vec![0; n];
    let mut result = Vec::new();
    permutation(0, n, &mut used, &mut perm, &mut result);
    result
}

fn permutation(pos: usize, n: usize, used: &mut Vec<bool>, perm: &mut Vec<usize>, result: &mut Vec<Vec<usize>>) {
    if pos == n {
        result.push(perm.clone());
        return
    }

    for i in 0..n {
        if !used[i] {
            perm[pos] = i;
            used[i] = true;
            permutation(pos + 1, n, used, perm, result);
            used[i] = false;
        }
    }
}

#[test]
fn test() {
    let cases: Vec<(&str, &str)> = vec![
        (
            r"4 330
0 1 10 100
1 0 20 200
10 20 0 300
100 200 300 0",
            "2"
        ),
        (
            r"5 5
0 1 1 1 1
1 0 1 1 1
1 1 0 1 1
1 1 1 0 1
1 1 1 1 0",
            "24"
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
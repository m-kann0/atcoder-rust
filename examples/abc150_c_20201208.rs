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
    let p: Vec<usize> = (0..n).map(|_| iterator.next().unwrap().parse::<usize>().unwrap() - 1).collect();
    let q: Vec<usize> = (0..n).map(|_| iterator.next().unwrap().parse::<usize>().unwrap() - 1).collect();

    let perms = permutation::generate(n);
    let mut a = 0;
    let mut b = 0;
    for i in 0..perms.len() {
        if perms[i] == p {
            a = i;
        }
        if perms[i] == q {
            b = i;
        }
    }
    let ans = (a as isize - b as isize).abs();
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
            r"3
1 3 2
3 1 2",
            "3"
        ),
        (
            r"8
7 3 5 4 2 1 6 8
3 8 2 5 4 6 7 1",
            "17517"
        ),
        (
            r"3
1 2 3
1 2 3",
            "0"
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
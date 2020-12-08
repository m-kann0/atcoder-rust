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
    let mut x = Vec::new();
    let mut y = Vec::new();
    for _ in 0..n {
        let xi: isize = iterator.next().unwrap().parse().unwrap();
        let yi: isize = iterator.next().unwrap().parse().unwrap();
        x.push(xi);
        y.push(yi);
    }

    let mut total: f64 = 0f64;
    let perms = permutation::generate(n);
    for p in &perms {
        for i in 1..n {
            let s = p[i];
            let t = p[i - 1];
            total += (((x[s] - x[t]) * (x[s] - x[t]) + (y[s] - y[t]) * (y[s] - y[t])) as f64).sqrt();
        }
    }

    let ans = total / perms.len() as f64;
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
0 0
1 0
0 1",
            "2.2761423749"
        ),
        (
            r"2
-879 981
-866 890",
            "91.9238815543"
        ),
        (
            r"8
-406 10
512 859
494 362
-955 -475
128 553
-986 -885
763 77
449 310",
            "7641.9817824387"
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
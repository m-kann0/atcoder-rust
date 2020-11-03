use std::io::Read;

fn main() {
    let mut buf = String::new();
    std::io::stdin().read_to_string(&mut buf).unwrap();

    let answer = solve(&buf);

    println!("{}", answer);
}

fn solve(input: &str) -> String {
    let mut iterator = input.split_whitespace();

    let k: usize = iterator.next().unwrap().parse().unwrap();
    let mut a: Vec<Vec<usize>> = Vec::new();
    for _ in 0..k {
        let n: usize = iterator.next().unwrap().parse().unwrap();
        let mut ai: Vec<usize> = Vec::new();
        for _ in 0..n {
            let aij: usize = iterator.next().unwrap().parse().unwrap();
            ai.push(aij);
        }
        a.push(ai);
    }
    let q: usize = iterator.next().unwrap().parse().unwrap();
    let b: Vec<usize> = (0..q).map(|_| iterator.next().unwrap().parse().unwrap()).collect();

    const MOD: usize = 1_000_000_000;

    let mut ans: usize = 0;
    let mut count: Vec<usize> = vec![0; 21];
    let mut used: Vec<bool> = vec![false; k];
    let mut r: Vec<usize> = vec![0; k];
    let mut counts: Vec<Vec<usize>> = vec![vec![0; 21]; k];



    for i in 0..q {
        let bi = b[i] - 1;
        if !used[bi] {
            for &aij in &a[bi] {
                ans = (ans + count[aij]) % MOD;
            }
        } else {
            for j in 1..21 {
                ans = (ans + count[j] * (counts[bi][j - 1] - counts[bi][j])) % MOD;
            }
        }
        if !used[bi] {
            let mut ri: usize = 0;
            for &aij in &a[bi] {
                ri = (ri + counts[bi][aij]) % MOD;
                for j in 0..aij {
                    counts[bi][j] = (counts[bi][j] + 1) % MOD;
                }
            }
            r[bi] = ri;
            used[bi] = true;
        }
        for j in 0..21 {
            count[j] = (count[j] + counts[bi][j]) % MOD;
        }
        ans = (ans + r[bi]) % MOD;
    }
    ans.to_string()
}

#[test]
fn test() {
    let cases: Vec<(&str, &str)> = vec![
        (
            r"2
3
1 3 2
2
5 4
2
1 2",
            "2"
        ),
        (
            r"10
8
16 6 15 10 18 13 17 11
13
4 10 6 4 14 17 13 9 3 9 4 8 14
11
11 17 12 3 13 8 10 11 18 2 19
10
18 11 16 19 4 17 7 3 5 8
3
3 10 9
13
8 17 20 8 20 1 5 17 4 16 18 20 4
15
11 2 1 16 8 17 4 7 3 6 4 13 16 16 16
2
12 12
8
7 14 7 5 8 17 19 4
15
3 6 1 16 11 5 3 15 9 15 12 15 5 19 7
20
4 3 7 6 1 8 2 3 9 8 6 3 10 9 7 7 3 2 2 10",
            "12430"
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
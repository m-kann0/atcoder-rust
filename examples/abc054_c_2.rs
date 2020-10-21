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
    let mut graph: Vec<HashSet<usize>> = vec![HashSet::new(); n];
    for _ in 0..m {
        let a: usize = iterator.next().unwrap().parse().unwrap();
        let b: usize = iterator.next().unwrap().parse().unwrap();
        graph[a - 1].insert(b - 1);
        graph[b - 1].insert(a - 1);
    }

    let mut ans: usize = 0;
    let perms = generate_permutation(n);
    for perm in perms {
        if perm[0] != 0 {
            continue;
        }
        let mut is_ok = true;
        for i in 1..n {
            if !graph[perm[i - 1]].contains(&perm[i]) {
                is_ok = false;
                break;
            }
        }
        if is_ok {
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
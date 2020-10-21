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
    let k: usize = iterator.next().unwrap().parse().unwrap();
    let cards: Vec<usize> = (0..n).map(|_| iterator.next().unwrap().parse().unwrap()).collect();

    let perms = generate_permutation(k);

    let mut generated: HashSet<String> = HashSet::new();
    for bit in 0..(1<<n) {
        let mut selected = Vec::new();
        for i in 0..n {
            if bit & (1 << i) > 0 {
                selected.push(cards[i]);
            }
        }
        if selected.len() != k {
            continue;
        }
        for p in &perms {
            let mut g = String::new();
            for i in p {
                g.push_str(&format!("{}", selected[*i]));
            }
            generated.insert(g);
        }
    }
    generated.len().to_string()
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
            r"4
2
1
2
12
1",
            "7"
        ),
        (
            r"6
3
72
2
12
7
2
1",
            "68"
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
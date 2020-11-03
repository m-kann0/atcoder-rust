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
    let s: Vec<char> = iterator.next().unwrap().chars().collect();

    let perms = generate_permutation(n);
    for perm in perms {
        let mut t: Vec<char> = Vec::new();
        for i in perm {
            t.push(s[i]);
        }
        if s == t {
            continue;
        }
        t.reverse();
        if s == t {
            continue;
        }
        t.reverse();
        let ans: String = t.iter().collect::<String>();
        return ans;
    }
    "None".to_string()
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
            r"3
cba",
            "acb"
        ),
        (
            r"2
aa",
            "None"
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
use std::io::Read;
use std::collections::HashMap;

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
    let s: Vec<char> = iterator.next().unwrap().chars().collect();

    let mut remain = s.clone();
    remain.sort();
    let mut t: Vec<char> = Vec::new();
    for i in 0..n {
        for j in 0..remain.len() {
            let mut t1 = t.clone();
            t1.push(remain[j]);

            let mut d = 0;
            for a in 0..=i {
                if s[a] != t1[a] {
                    d += 1;
                }
            }

            if d > k {
                continue;
            }

            let mut counts: HashMap<char, usize> = HashMap::new();
            for a in 0..remain.len() {
                if a == j {
                    continue;
                }
                *counts.entry(remain[a]).or_insert(0) += 1;
            }

            let mut d2 = 0;
            for a in (i + 1)..n {
                if counts.contains_key(&s[a]) && *counts.get(&s[a]).unwrap() > 0 {
                    counts.insert(s[a], *counts.get(&s[a]).unwrap() - 1);
                } else {
                    d2 += 1;
                }
            }

            if d + d2 <= k {
                t = t1;
                remain.remove(j);
                break;
            }
        }
    }
    let result: String = t.iter().collect::<String>();
    result
}

#[test]
fn test() {
    let cases: Vec<(&str, &str)> = vec![
        (
            r"3 2
abc",
            "abc"
        ),
        (
            r"7 2
atcoder",
            "actoder"
        ),
        (
            r"7 7
atcoder",
            "acdeort"
        ),
        (
            r"10 3
helloworld",
            "dehloworll"
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
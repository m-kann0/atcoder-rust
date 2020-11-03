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
    let mut counts: HashMap<String, usize> = HashMap::new();
    for _ in 0..n {
        let s = iterator.next().unwrap().to_string();
        *counts.entry(s).or_insert(0) += 1;
    }

    let mut ranking: Vec<(usize, String)> = Vec::new();
    for entry in counts {
        ranking.push((entry.1, entry.0));
    }
    ranking.sort();
    ranking.reverse();

    let k = k - 1;
    if k == 0 {
        if ranking.len() >= 2 && ranking[k].0 == ranking[k + 1].0 {
            return "AMBIGUOUS".to_string();
        }
        return ranking[k].clone().1;
    }

    if k == ranking.len() - 1 {
        if ranking[k - 1].0 == ranking[k].0 {
            return "AMBIGUOUS".to_string();
        }
        return ranking[k].clone().1;
    }

    if ranking[k - 1].0 == ranking[k].0 || ranking[k].0 == ranking[k + 1].0 {
        return "AMBIGUOUS".to_string();
    }
    return ranking[k].clone().1;
}

#[test]
fn test() {
    let cases: Vec<(&str, &str)> = vec![
        (
            r"6 2
abcde
caac
abcde
caac
abc
caac",
            "abcde"
        ),
        (
            r"9 3
a
a
bb
bb
a
ccc
bb
ccc
dddd",
            "ccc"
        ),
        (
            r"7 2
caac
abcde
caac
abc
abcde
caac
abc",
            "AMBIGUOUS"
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
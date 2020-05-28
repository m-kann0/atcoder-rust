use std::io::Read;
use std::collections::{HashSet, HashMap};
use std::iter::FromIterator;

fn main() {
    let mut buf = String::new();
    std::io::stdin().read_to_string(&mut buf).unwrap();

    let answer = solve(&buf);

    println!("{}", answer);
}

fn solve(input: &str) -> String {
    let mut iterator = input.split_whitespace();

    let n: usize = iterator.next().unwrap().parse().unwrap();
    let a: Vec<usize> = (0..n).map(|_| iterator.next().unwrap().parse().unwrap()).collect();

    let s: HashSet<usize> = HashSet::from_iter(a.clone());
    let mut t: Vec<usize> = Vec::from_iter(s);
    t.sort();
    let mut table: HashMap<usize, usize> = HashMap::new();
    for (bi, ai) in t.iter().enumerate() {
        table.insert(*ai, bi);
    }

    let b: Vec<usize> = a.iter().map(|ai| *table.get(ai).unwrap()).collect();
    return b.iter().map(|it| format!("{}", it)).collect::<Vec<String>>().join("\n").trim().to_string();
}

#[test]
fn test() {
    let cases: Vec<(&str, &str)> = vec![
        (
            r"5
3
3
1
6
1",
            r"1
1
0
2
0"
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
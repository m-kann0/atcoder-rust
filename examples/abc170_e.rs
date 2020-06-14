use std::io::Read;
use std::collections::BTreeSet;

fn main() {
    let mut buf = String::new();
    std::io::stdin().read_to_string(&mut buf).unwrap();

    let answer = solve(&buf);

    println!("{}", answer);
}

fn solve(input: &str) -> String {
    let mut iterator = input.split_whitespace();

    let n: usize = iterator.next().unwrap().parse().unwrap();
    let q: usize = iterator.next().unwrap().parse().unwrap();
    let mut a: Vec<usize> = Vec::new();
    let mut s: Vec<BTreeSet<(usize, usize)>> = vec![BTreeSet::new(); 200_000];
    let mut belongs: Vec<usize> = Vec::new();
    for i in 0..n {
        let ai: usize = iterator.next().unwrap().parse().unwrap();
        let bi: usize = iterator.next().unwrap().parse().unwrap();
        let bi = bi - 1;
        a.push(ai);
        s[bi].insert((ai, i));
        belongs.push(bi);
    }

    let mut m: BTreeSet<(usize, usize)> = BTreeSet::new();
    for i in 0..200_000 {
        if !s[i].is_empty() {
            m.insert((s[i].iter().rev().next().unwrap().0, i));
        }
    }

    let mut result = String::new();
    for _ in 0..q {
        let ci: usize = iterator.next().unwrap().parse().unwrap();
        let di: usize = iterator.next().unwrap().parse().unwrap();
        let ci = ci - 1;
        let di = di - 1;
        let ai: usize = a[ci];
        let bi: usize = belongs[ci];

        m.remove(&(s[bi].iter().rev().next().unwrap().0, bi));
        if !s[di].is_empty() {
            m.remove(&(s[di].iter().rev().next().unwrap().0, di));
        }

        s[bi].remove(&(ai, ci));
        belongs[ci] = di;
        s[di].insert((ai, ci));

        if !s[bi].is_empty() {
            m.insert((s[bi].iter().rev().next().unwrap().0, bi));
        }
        m.insert((s[di].iter().rev().next().unwrap().0, di));

        let equality: usize = m.iter().next().unwrap().0;
        result.push_str(&format!("{}\n", equality));
    }

    return result.trim().to_string();
}

#[test]
fn test() {
    let cases: Vec<(&str, &str)> = vec![
        (
            r"6 3
8 1
6 2
9 3
1 1
2 2
1 3
4 3
2 1
1 2",
            "6
2
6"
        ),
        (
            r"2 2
4208 1234
3056 5678
1 2020
2 2020",
            "3056
4208"
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
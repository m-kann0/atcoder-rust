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
    let mut t: Vec<usize> = Vec::new();
    let mut count: HashMap<usize, usize> = HashMap::new();
    for _ in 0..n {
        let ti: usize = iterator.next().unwrap().parse().unwrap();
        t.push(ti);
        *count.entry(ti).or_insert(0) += 1;
    }

    t.sort();

    let mut time: usize = 0;
    let mut p: usize = 0;
    for ti in t {
        time += ti;
        p += time;
    }

    let mut q: usize = 1;
    const MOD: usize = 1_000_000_007;
    for (_k, mut v) in count {
        while v > 1 {
            q = q * v % MOD;
            v -= 1;
        }
    }

    return format!("{}\n{}", p, q);
}

#[test]
fn test() {
    let cases: Vec<(&str, &str)> = vec![
        (
            r"2
20
10",
            "40
1"
        ),
        (
            r"5
2
1
2
1
2",
            "21
12"
        ),
        (
            r"13
1
1
1
1
1
1
1
1
1
1
1
1
1",
            "91
227020758"
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
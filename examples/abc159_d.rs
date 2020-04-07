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
    let mut a: Vec<usize> = Vec::new();
    let mut counts: HashMap<usize, usize> = HashMap::new();
    for _ in 0..n {
        let ai: usize = iterator.next().unwrap().parse().unwrap();
        a.push(ai);
        *counts.entry(ai).or_insert(0) += 1;
    }

    let mut total = 0;
    for (_, &v) in &counts {
        if v >= 2 {
            total += comb(v, 2);
        }
    }

    let mut result = String::new();

    for ai in a {
        let mut ans = total;
        let count = *counts.get(&ai).unwrap();
        if count >= 2 {
            ans -= comb(count, 2);
        }
        if count >= 3 {
            ans += comb(count - 1, 2);
        }
        result.push_str(&format!("{}\n", ans));
    }

    return result.trim().to_string();
}

fn comb(n: usize, r: usize) -> usize {
    let mut n = n;
    let mut r = if r > n / 2 { n - r } else { r };

    let mut numerator = 1;
    let mut denominator = 1;
    while r > 0 {
        numerator *= n;
        denominator *= r;

        n -= 1;
        r -= 1;
    }
    numerator / denominator
}

#[test]
fn test() {
    let cases: Vec<(&str, &str)> = vec![
        (
            r"5
1 1 2 1 2",
            "2
2
3
2
3"
        ),
        (
            r"4
1 2 3 4",
            "0
0
0
0"
        ),
        (
            r"5
3 3 3 3 3",
            "6
6
6
6
6"
        ),
        (
            r"8
1 2 1 4 2 1 4 1",
            "5
7
5
7
7
5
7
5"
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
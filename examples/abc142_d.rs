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

    let a: usize = iterator.next().unwrap().parse().unwrap();
    let b: usize = iterator.next().unwrap().parse().unwrap();

    let gcd: usize = gcd(a, b);

    let pf = prime_factorization(gcd);

    let ans = pf.len() + 1;
    return ans.to_string();
}

fn gcd(mut x: usize, mut y: usize) -> usize {
    while y > 0 {
        let r = x % y;
        x = y;
        y = r;
    }
    return x;
}

fn prime_factorization(mut n: usize) -> HashMap<usize, usize> {
    let mut result: HashMap<_, _> = HashMap::new();

    for d in 2.. {
        if d * d > n {
            break;
        }
        while n % d == 0 {
            *result.entry(d).or_insert(0) += 1;
            n /= d;
        }
    }

    if n != 1 {
        result.insert(n, 1);
    }

    return result;
}

#[test]
fn test() {
    let cases: Vec<(&str, &str)> = vec![
        (
            r"12 18",
            "3"
        ),
        (
            r"420 660",
            "4"
        ),
        (
            r"1 2019",
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
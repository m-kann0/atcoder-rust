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

    let factors: HashMap<usize, usize> = prime_factorization(n);

    let mut ans: usize = 0;
    for (_, v) in factors {
        ans += fact(v);
    }
    return ans.to_string();
}

fn fact(mut n: usize) -> usize {
    let mut i = 1;
    loop {
        if n >= i {
            n -= i;
            i += 1;
        } else {
            return i - 1;
        }
    }
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
            r"24",
            "3"
        ),
        (
            r"1",
            "0"
        ),
        (
            r"64",
            "3"
        ),
        (
            r"1000000007",
            "1"
        ),
        (
            r"997764507000",
            "7"
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
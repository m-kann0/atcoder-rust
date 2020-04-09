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
    let a: Vec<usize> = (0..n)
        .map(|_| iterator.next().unwrap().parse().unwrap())
        .collect();

    let ans: usize = gcd(&a);

    return ans.to_string();
}

fn gcd(vec: &Vec<usize>) -> usize {
    if vec.len() < 2 {
        panic!();
    }
    let mut result = vec[0];
    for i in 1..vec.len() {
        result = _gcd(result, vec[i]);
    }
    result
}

fn _gcd(m: usize, n: usize) -> usize {
    if n > m {
        return _gcd(n, m);
    }
    if n == 0 {
        return m;
    }
    return _gcd(n, m % n);
}

#[test]
fn test() {
    let cases: Vec<(&str, &str)> = vec![
        (
            r"4
2 10 8 40",
            "2"
        ),
        (
            r"4
5 13 8 1000000000",
            "1"
        ),
        (
            r"3
1000000000 1000000000 1000000000",
            "1000000000"
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
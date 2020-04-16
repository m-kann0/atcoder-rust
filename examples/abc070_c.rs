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

    let mut l: usize = iterator.next().unwrap().parse().unwrap();
    for _ in 0..(n - 1) {
        let t: usize = iterator.next().unwrap().parse().unwrap();
        l = l / gcd(l, t) * t;
    }
    return l.to_string();
}

fn gcd(mut p: usize, mut q: usize) -> usize {
    while q != 0 {
        let r = p % q;
        p = q;
        q = r;
    }
    return p;
}

#[test]
fn test() {
    let cases: Vec<(&str, &str)> = vec![
        (
            r"2
2
3",
            "6"
        ),
        (
            r"5
2
5
10
1000000000000000000
1000000000000000000",
            "1000000000000000000"
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
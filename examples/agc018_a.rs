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
    let k: usize = iterator.next().unwrap().parse().unwrap();
    let a: Vec<usize> = (0..n).map(|_| iterator.next().unwrap().parse().unwrap()).collect();

    if k > *a.iter().max().unwrap() {
        return "IMPOSSIBLE".to_string();
    }

    let mut g = a[0];
    for i in 1..n {
        g = gcd(g, a[i]);
    }

    if k % g == 0 {
        "POSSIBLE".to_string()
    } else {
        "IMPOSSIBLE".to_string()
    }
}

fn gcd(mut x: usize, mut y: usize) -> usize {
    while y > 0 {
        let r = x % y;
        x = y;
        y = r;
    }
    return x;
}

#[test]
fn test() {
    let cases: Vec<(&str, &str)> = vec![
        (
            r"3 7
9 3 4",
            "POSSIBLE"
        ),
        (
            r"3 5
6 9 3",
            "IMPOSSIBLE"
        ),
        (
            r"4 11
11 3 7 15",
            "POSSIBLE"
        ),
        (
            r"5 12
10 2 8 6 4",
            "IMPOSSIBLE"
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
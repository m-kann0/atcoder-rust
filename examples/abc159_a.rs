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
    let m: usize = iterator.next().unwrap().parse().unwrap();

    let ans = comb(n, 2) + comb(m, 2);
    return ans.to_string();
}

fn comb(mut n: usize, mut r: usize) -> usize {
    if n < r {
        return 0;
    }
    let mut numerator = 1;
    let mut denominator = 1;
    while r > 0 {
        numerator *= n;
        denominator *= r;
        n -= 1;
        r -= 1;
    }
    return numerator / denominator;
}

#[test]
fn test() {
    let cases: Vec<(&str, &str)> = vec![
        (
            r"2 1",
            "1"
        ),
        (
            r"4 3",
            "9"
        ),
        (
            r"1 1",
            "0"
        ),
        (
            r"13 3",
            "81"
        ),
        (
            r"0 3",
            "3"
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
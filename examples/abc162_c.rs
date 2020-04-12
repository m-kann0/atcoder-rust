use std::io::Read;

fn main() {
    let mut buf = String::new();
    std::io::stdin().read_to_string(&mut buf).unwrap();

    let answer = solve(&buf);

    println!("{}", answer);
}

fn solve(input: &str) -> String {
    let mut iterator = input.split_whitespace();

    let k: usize = iterator.next().unwrap().parse().unwrap();

    let mut ans: usize = 0;
    for a in 1..=k {
        for b in 1..=k {
            for c in 1..=k {
                ans += gcd(gcd(a, b), c);
            }
        }
    }

    return ans.to_string();
}

fn gcd(mut x: usize, mut y: usize) -> usize {
    while y != 0 {
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
            r"2",
            "9"
        ),
        (
            r"200",
            "10813692"
        ),
        (
            r"1",
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
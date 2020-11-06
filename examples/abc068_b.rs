use std::io::Read;
use std::cmp::max;

fn main() {
    let mut buf = String::new();
    std::io::stdin().read_to_string(&mut buf).unwrap();

    let answer = solve(&buf);

    println!("{}", answer);
}

fn solve(input: &str) -> String {
    let mut iterator = input.split_whitespace();

    let n: usize = iterator.next().unwrap().parse().unwrap();

    let mut ans: usize = 0;
    let mut y: usize = 0;
    for i in 1..=n {
        let mut x = i;
        let mut a = 0;
        while x > 0 {
            if x % 2 == 0 {
                a += 1;
                x /= 2;
            } else {
                break;
            }
        }
        if a >= ans {
            ans = max(ans, a);
            y = i;
        }
    }
    y.to_string()
}

#[test]
fn test() {
    let cases: Vec<(&str, &str)> = vec![
        (
            r"7",
            "4"
        ),
        (
            r"32",
            "32"
        ),
        (
            r"1",
            "1"
        ),
        (
            r"100",
            "64"
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
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

    let mut ans: usize = 1;
    let mut increasing: bool = false;
    let mut decreasing: bool = false;

    let mut prev: usize = 0;
    for _ in 0..n {
        let a: usize = iterator.next().unwrap().parse().unwrap();

        // println!("prev = {}, a = {}, inc = {}, dec = {}", prev, a, increasing, decreasing);

        if prev == 0 {
            // do nothing
        } else if !increasing && !decreasing {
            if a < prev {
                decreasing = true;
            } else if a > prev {
                increasing = true;
            }
        } else if increasing && a < prev {
            ans += 1;
            increasing = false;
        } else if decreasing && a > prev {
            ans += 1;
            decreasing = false;
        }

        prev = a;

        // println!("ans = {}", ans);
    }

    return ans.to_string();
}

#[test]
fn test() {
    let cases: Vec<(&str, &str)> = vec![
        (
            r"6
1 2 3 2 2 1",
            "2"
        ),
        (
            r"9
1 2 1 2 1 2 1 2 1",
            "5"
        ),
        (
            r"7
1 2 3 2 1 999999999 1000000000",
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
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

    let mut ans: usize = 0;
    for b in 1..=n {
        if b <= k {
            continue;
        }
        let mut now = (b - k) * (n / b);
        if k == 0 {
            now += n % b
        } else if n % b > k - 1 {
            now += n % b - (k - 1)
        }
        ans += now;
    }
    ans.to_string()
}

fn solve2(input: &str) -> String {
    let mut iterator = input.split_whitespace();

    let n: usize = iterator.next().unwrap().parse().unwrap();
    let k: usize = iterator.next().unwrap().parse().unwrap();

    let mut ans: usize = 0;
    for a in 1..=n {
        for b in 1..=n {
            if a % b >= k {
                ans += 1;
            }
        }
    }
    ans.to_string()
}

#[test]
fn test2() {
    for n in 1..100 {
        for k in 0..n {
            let input = &format!("{} {}", n, k);
            let ans1 = solve(input);
            let ans2 = solve2(input);
            if ans1 != ans2 {
                println!("input: {}, ans1: {}, ans2: {}", input, ans1, ans2);
            }
        }
    }
}

#[test]
fn test() {
    let cases: Vec<(&str, &str)> = vec![
        (
            r"5 2",
            "7"
        ),
        (
            r"10 0",
            "100"
        ),
        (
            r"5 1",
            "15"
        ),
        (
            r"31415 9265",
            "287927211"
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
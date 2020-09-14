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
    let k: usize = iterator.next().unwrap().parse().unwrap();
    let s: Vec<usize> = (0..n).map(|_| iterator.next().unwrap().parse().unwrap()).collect();

    if s.contains(&0) {
        return n.to_string();
    }

    let mut ans = 0;
    let mut current = 1;
    let mut right = 0;
    for left in 0..n {
        while right < n && current * s[right] <= k {
            current *= s[right];
            right += 1;
        }
        ans = max(ans, right - left);
        if left == right {
            right += 1;
        } else {
            current /= s[left];
        }
    }
    ans.to_string()
}

#[test]
fn test() {
    let cases: Vec<(&str, &str)> = vec![
        (
            r"7 6
4
3
1
1
2
10
2",
            "4"
        ),
        (
            r"6 10
10
10
10
10
0
10",
            "6"
        ),
        (
            r"6 9
10
10
10
10
10
10",
            "0"
        ),
        (
            r"4 0
1
2
3
4",
            "0"
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
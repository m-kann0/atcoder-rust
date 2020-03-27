use std::io::Read;
use std::cmp::min;

fn main() {
    let mut buf = String::new();
    std::io::stdin().read_to_string(&mut buf).unwrap();

    let answer = solve(&buf);

    println!("{}", answer);
}

fn solve(input: &str) -> String {
    let mut iterator = input.split_whitespace();

    let n: usize = iterator.next().unwrap().parse().unwrap();
    let k: Vec<usize> = (0..n-1)
        .map(|_| iterator.next().unwrap().parse().unwrap())
        .collect();

    let mut ans: Vec<usize> = vec![0; n];
    ans[0] = k[0];
    ans[n - 1] = k[n - 2];
    for i in 1..n-1 {
        ans[i] = min(k[i - 1], k[i]);
    }

    return ans.iter().map(|a| a.to_string()).collect::<Vec<String>>().join(" ");
}

#[test]
fn test() {
    let cases: Vec<(&str, &str)> = vec![
        (
            r"4
3 5 5",
            "1 3 5 4"
        ),
        (
            r"6
4 8 8 2 5",
            "4 4 8 2 2 5"
        ),
        (
            r"5
1 2 3 4",
            "1 1 2 3 4"
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
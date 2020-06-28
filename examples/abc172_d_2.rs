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

    let mut ans: usize = 0;
    for i in 1..=n {
        ans += i * g(n / i);
    }
    return ans.to_string();
}

fn g(n: usize) -> usize {
    return n * (n + 1) / 2;
}

#[test]
fn test() {
    let cases: Vec<(&str, &str)> = vec![
        (
            r"4",
            "23"
        ),
        (
            r"100",
            "26879"
        ),
        (
            r"10000000",
            "838627288460105"
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
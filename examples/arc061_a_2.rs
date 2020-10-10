use std::io::Read;

fn main() {
    let mut buf = String::new();
    std::io::stdin().read_to_string(&mut buf).unwrap();

    let answer = solve(&buf);

    println!("{}", answer);
}

fn solve(input: &str) -> String {
    let mut iterator = input.split_whitespace();

    let s: Vec<usize> = iterator.next().unwrap().chars().map(|c| c.to_digit(10).unwrap() as usize).collect();

    let mut sum: usize = 0;
    for i in 0..(1 << (s.len() - 1)) {
        let mut plus: Vec<bool> = vec![false; s.len() - 1];
        for j in 0..(s.len() - 1) {
            plus[j] = i & (1 << j) > 0;
        }

        let mut current = 0;
        for j in 0..(s.len() - 1) {
            current *= 10;
            current += s[j];
            if plus[j] {
                sum += current;
                current = 0;
            }
        }
        current *= 10;
        current += s.last().unwrap();
        sum += current;
    }
    sum.to_string()
}

#[test]
fn test() {
    let cases: Vec<(&str, &str)> = vec![
        (
            r"125",
            "176"
        ),
        (
            r"9999999999",
            "12656242944"
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
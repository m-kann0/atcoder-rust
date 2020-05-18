use std::io::Read;

fn main() {
    let mut buf = String::new();
    std::io::stdin().read_to_string(&mut buf).unwrap();

    let answer = solve(&buf);

    println!("{}", answer);
}

fn solve(input: &str) -> String {
    let mut iterator = input.split_whitespace();

    let a: Vec<char> = iterator.next().unwrap().chars().collect();
    let n = a.len();

    let mut diff: usize = 0;
    for i in 0..(n / 2) {
        if a[i] != a[n - 1 - i] {
            diff += 1;
        }
    }

    let ans: usize = if diff == 0 {
        if n % 2 == 0 {
            25 * n
        } else {
            25 * (n - 1)
        }
    } else if diff == 1 {
        24 * 2 + 25 * (n - 2)
    } else {
        25 * n
    };
    return ans.to_string();
}

#[test]
fn test() {
    let cases: Vec<(&str, &str)> = vec![
        (
            r"ARC",
            "73"
        ),
        (
            r"S",
            "0"
        ),
        (
            r"NOLEMONNOMELON",
            "350"
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
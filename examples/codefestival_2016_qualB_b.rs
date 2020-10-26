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
    let a: usize = iterator.next().unwrap().parse().unwrap();
    let b: usize = iterator.next().unwrap().parse().unwrap();
    let s: Vec<char> = iterator.next().unwrap().chars().collect();

    let mut result = String::new();
    let mut x = 0;
    let mut y = 0;
    for i in 0..n {
        if s[i] == 'a' {
            if x < a + b {
                x += 1;
                result.push_str("Yes\n");
            } else {
                result.push_str("No\n");
            }
        } else if s[i] == 'b' {
            if x < a + b && y < b {
                x += 1;
                y += 1;
                result.push_str("Yes\n");
            } else {
                result.push_str("No\n");
            }
        } else {
            result.push_str("No\n");
        }
    }
    result.trim().to_string()
}

#[test]
fn test() {
    let cases: Vec<(&str, &str)> = vec![
        (
            r"10 2 3
abccabaabb",
            "Yes
Yes
No
No
Yes
Yes
Yes
No
No
No"
        ),
        (
            r"12 5 2
cabbabaacaba",
            "No
Yes
Yes
Yes
Yes
No
Yes
Yes
No
Yes
No
No"
        ),
        (
            r"5 2 2
ccccc",
            "No
No
No
No
No"
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
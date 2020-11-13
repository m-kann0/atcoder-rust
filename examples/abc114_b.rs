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

    let s: Vec<isize> = iterator.next().unwrap().chars().map(|c| c as isize - '0' as isize).collect();

    let mut ans = 753;
    for i in 0..s.len() {
        if i + 2 >= s.len() {
            break;
        }
        let now = s[i] * 100 + s[i + 1] * 10 + s[i + 2];
        ans = min(ans, (753 - now).abs());
    }
    ans.to_string()
}

#[test]
fn test() {
    let cases: Vec<(&str, &str)> = vec![
        (
            r"1234567876",
            "34"
        ),
        (
            r"35753",
            "0"
        ),
        (
            r"1111111111",
            "642"
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
use std::io::Read;

fn main() {
    let mut buf = String::new();
    std::io::stdin().read_to_string(&mut buf).unwrap();

    let answer = solve(&buf);

    println!("{}", answer);
}

fn solve(input: &str) -> String {
    let mut iterator = input.split_whitespace();

    let mut n: isize = iterator.next().unwrap().parse().unwrap();

    if n == 0 {
        return "0".to_string();
    }

    let mut ans = String::new();
    while n != 0 {
        if (n % 2).abs() == 1 {
            ans = format!("{}{}", 1, ans);
            n -= 1;
        } else {
            ans = format!("{}{}", 0, ans);
        }
        n /= -2;
    }
    ans
}

#[test]
fn test() {
    let cases: Vec<(&str, &str)> = vec![
        (
            r"-9",
            "1011"
        ),
        (
            r"123456789",
            "11000101011001101110100010101"
        ),
        (
            r"0",
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
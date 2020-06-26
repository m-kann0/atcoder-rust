use std::io::Read;

fn main() {
    let mut buf = String::new();
    std::io::stdin().read_to_string(&mut buf).unwrap();

    let answer = solve(&buf);

    println!("{}", answer);
}

fn solve(input: &str) -> String {
    let mut iterator = input.split_whitespace();

    let s1 = iterator.next().unwrap();
    let s2 = iterator.next().unwrap();
    let s3 = iterator.next().unwrap();

    let ans = format!(
        "{}{}{}",
        s1.chars().next().unwrap(),
        s2.chars().next().unwrap(),
        s3.chars().next().unwrap()
    );

    return ans;
}

#[test]
fn test() {
    let cases: Vec<(&str, &str)> = vec![
        (
            r"AtCoder Beginner Contest",
            "ABC"
        ),
        (
            r"AtCoder Snuke Contest",
            "ASC"
        ),
        (
            r"AtCoder X Contest",
            "AXC"
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
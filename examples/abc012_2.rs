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

    let h = n / 3600;
    let m = n % 3600 / 60;
    let s = n % 3600 % 60;
    return format!("{:0>2}:{:0>2}:{:0>2}", h, m, s);
}

#[test]
fn test() {
    let cases: Vec<(&str, &str)> = vec![
        (
            r"3661",
            "01:01:01"
        ),
        (
            r"86399",
            "23:59:59"
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
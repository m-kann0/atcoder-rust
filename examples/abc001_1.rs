use std::io::Read;

fn main() {
    let mut buf = String::new();
    std::io::stdin().read_to_string(&mut buf).unwrap();

    let answer = solve(&buf);

    println!("{}", answer);
}

fn solve(input: &str) -> String {
    let mut iterator = input.split_whitespace();

    let h1: isize = iterator.next().unwrap().parse().unwrap();
    let h2: isize = iterator.next().unwrap().parse().unwrap();

    return (h1 - h2).to_string();
}

#[test]
fn test() {
    let cases: Vec<(&str, &str)> = vec![
        ("15 10", "5"),
        ("0 0", "0"),
        ("5 20", "-15"),
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
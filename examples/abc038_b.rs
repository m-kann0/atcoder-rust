use std::io::Read;

fn main() {
    let mut buf = String::new();
    std::io::stdin().read_to_string(&mut buf).unwrap();

    let answer = solve(&buf);

    println!("{}", answer);
}

fn solve(input: &str) -> String {
    let mut iterator = input.split_whitespace();

    let h1: usize = iterator.next().unwrap().parse().unwrap();
    let w1: usize = iterator.next().unwrap().parse().unwrap();
    let h2: usize = iterator.next().unwrap().parse().unwrap();
    let w2: usize = iterator.next().unwrap().parse().unwrap();

    if h1 == h2 || h1 == w2 || w1 == h2 || w1 == w2 {
        return "YES".to_string();
    }

    return "NO".to_string();
}

#[test]
fn test() {
    let cases: Vec<(&str, &str)> = vec![
        (
            r"1080 1920
1080 1920",
            "YES"
        ),
        (
            r"1080 1920
1920 1080",
            "YES"
        ),
        (
            r"334 668
668 1002",
            "YES"
        ),
        (
            r"100 200
300 150",
            "NO"
        ),
        (
            r"120 120
240 240",
            "NO"
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
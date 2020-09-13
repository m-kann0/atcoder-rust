use std::io::Read;

fn main() {
    let mut buf = String::new();
    std::io::stdin().read_to_string(&mut buf).unwrap();

    let answer = solve(&buf);

    println!("{}", answer);
}

fn solve(input: &str) -> String {
    let mut iterator = input.split_whitespace();

    let a: isize = iterator.next().unwrap().parse().unwrap();
    let b: isize = iterator.next().unwrap().parse().unwrap();

    let c: isize = iterator.next().unwrap().parse().unwrap();
    let d: isize = iterator.next().unwrap().parse().unwrap();

    vec![
        a * c,
        a * d,
        b * c,
        b * d,
    ].iter().max().unwrap().to_string()
}

#[test]
fn test() {
    let cases: Vec<(&str, &str)> = vec![
        (
            r"1 2 1 1",
            "2"
        ),
        (
            r"3 5 -4 -2",
            "-6"
        ),
        (
            r"-1000000000 0 -1000000000 0",
            "1000000000000000000"
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
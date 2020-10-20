use std::io::Read;

fn main() {
    let mut buf = String::new();
    std::io::stdin().read_to_string(&mut buf).unwrap();

    let answer = solve(&buf);

    println!("{}", answer);
}

fn solve(input: &str) -> String {
    let mut iterator = input.split_whitespace();

    let a1: isize = iterator.next().unwrap().parse().unwrap();
    let a2: isize = iterator.next().unwrap().parse().unwrap();
    let a3: isize = iterator.next().unwrap().parse().unwrap();

    let v = vec![
        (a1 - a2).abs() + (a2 - a3).abs(),
        (a1 - a3).abs() + (a3 - a2).abs(),
        (a2 - a1).abs() + (a1 - a3).abs(),
        (a2 - a3).abs() + (a3 - a1).abs(),
        (a3 - a1).abs() + (a1 - a2).abs(),
        (a3 - a2).abs() + (a2 - a1).abs(),
    ];
    let ans = v.iter().min().unwrap();
    ans.to_string()
}

#[test]
fn test() {
    let cases: Vec<(&str, &str)> = vec![
        (
            r"1 6 3",
            "5"
        ),
        (
            r"11 5 5",
            "6"
        ),
        (
            r"100 100 100",
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
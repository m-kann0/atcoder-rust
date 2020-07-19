use std::io::Read;

fn main() {
    let mut buf = String::new();
    std::io::stdin().read_to_string(&mut buf).unwrap();

    let answer = solve(&buf);

    println!("{}", answer);
}

fn solve(input: &str) -> String {
    let mut iterator = input.split_whitespace();

    let mut iterator = iterator.next().unwrap().split("/");
    let year: usize = iterator.next().unwrap().parse().unwrap();
    let month: usize = iterator.next().unwrap().parse().unwrap();
    let day: usize = iterator.next().unwrap().parse().unwrap();
    if month <= 4 {
        "Heisei".to_string()
    } else {
        "TBD".to_string()
    }
}

#[test]
fn test() {
    let cases: Vec<(&str, &str)> = vec![
        (
            r"2019/04/30",
            "Heisei"
        ),
        (
            r"2019/11/01",
            "TBD"
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
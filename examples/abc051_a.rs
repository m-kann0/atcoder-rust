use std::io::Read;

fn main() {
    let mut buf = String::new();
    std::io::stdin().read_to_string(&mut buf).unwrap();

    let answer = solve(&buf);

    println!("{}", answer);
}

fn solve(input: &str) -> String {
    let mut iterator = input.split_whitespace();

    let s = iterator.next().unwrap();

    return s.replace(",", " ");
}

#[test]
fn test() {
    let cases: Vec<(&str, &str)> = vec![
        (
            r"happy,newyear,enjoy",
            "happy newyear enjoy"
        ),
        (
            r"haiku,atcoder,tasks",
            "haiku atcoder tasks"
        ),
        (
            r"abcde,fghihgf,edcba",
            "abcde fghihgf edcba"
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
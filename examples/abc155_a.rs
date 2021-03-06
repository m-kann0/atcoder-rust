use std::io::Read;

fn main() {
    let mut buf = String::new();
    std::io::stdin().read_to_string(&mut buf).unwrap();

    let answer = solve(&buf);

    println!("{}", answer);
}

fn solve(input: &str) -> String {
    let mut iterator = input.split_whitespace();

    let a = iterator.next().unwrap().parse::<u8>().unwrap();
    let b = iterator.next().unwrap().parse::<u8>().unwrap();
    let c = iterator.next().unwrap().parse::<u8>().unwrap();

    if a == b && a != c {
        return String::from("Yes");
    }
    if a == c && a != b {
        return String::from("Yes");
    }
    if b == c && b != a {
        return String::from("Yes");
    }
    return String::from("No");
}

#[test]
fn test() {
    let cases: Vec<(&str, &str)> = vec![
        (
            r"5 7 5",
            "Yes"
        ),
        (
            r"4 4 4",
            "No"
        ),
        (
            r"4 9 6",
            "No"
        ),
        (
            r"3 3 4",
            "Yes"
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

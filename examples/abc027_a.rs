use std::io::Read;

fn main() {
    let mut buf = String::new();
    std::io::stdin().read_to_string(&mut buf).unwrap();

    let answer = solve(&buf);

    println!("{}", answer);
}

fn solve(input: &str) -> String {
    let mut iterator = input.split_whitespace();

    let l1: usize = iterator.next().unwrap().parse().unwrap();
    let l2: usize = iterator.next().unwrap().parse().unwrap();
    let l3: usize = iterator.next().unwrap().parse().unwrap();

    if l1 == l2 {
        return l3.to_string();
    }

    if l2 == l3 {
        return l1.to_string();
    }

    return l2.to_string();
}

#[test]
fn test() {
    let cases: Vec<(&str, &str)> = vec![
        (
            r"1 1 2",
            "2"
        ),
        (
            r"4 3 4",
            "3"
        ),
        (
            r"5 5 5",
            "5"
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
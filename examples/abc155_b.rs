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

    let mut approved = true;
    for _i in 0..n {
        let a: usize = iterator.next().unwrap().parse().unwrap();
        if a % 2 == 0 && !(a % 3 == 0 || a % 5 == 0) {
            approved = false;
            break;
        }
    }

    if approved {
        String::from("APPROVED")
    } else {
        String::from("DENIED")
    }
}

#[test]
fn test() {
    let cases: Vec<(&str, &str)> = vec![
        (
            r"5
6 7 9 10 31",
            "APPROVED"
        ),
        (
            r"3
28 27 24",
            "DENIED"
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

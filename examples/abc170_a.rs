use std::io::Read;

fn main() {
    let mut buf = String::new();
    std::io::stdin().read_to_string(&mut buf).unwrap();

    let answer = solve(&buf);

    println!("{}", answer);
}

fn solve(input: &str) -> String {
    let mut iterator = input.split_whitespace();

    let x1: usize = iterator.next().unwrap().parse().unwrap();
    let x2: usize = iterator.next().unwrap().parse().unwrap();
    let x3: usize = iterator.next().unwrap().parse().unwrap();
    let x4: usize = iterator.next().unwrap().parse().unwrap();
    let x5: usize = iterator.next().unwrap().parse().unwrap();

    if x1 == 0 {
        return "1".to_string();
    } else if x2 == 0 {
        return "2".to_string();
    } else if x3 == 0 {
        return "3".to_string();
    } else if x4 == 0 {
        return "4".to_string();
    }
    return "5".to_string();
}

#[test]
fn test() {
    let cases: Vec<(&str, &str)> = vec![
        (
            r"0 2 3 4 5",
            "1"
        ),
        (
            r"1 2 0 4 5",
            "3"
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
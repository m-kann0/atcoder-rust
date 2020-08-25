use std::io::Read;

fn main() {
    let mut buf = String::new();
    std::io::stdin().read_to_string(&mut buf).unwrap();

    let answer = solve(&buf);

    println!("{}", answer);
}

fn solve(input: &str) -> String {
    let mut iterator = input.split_whitespace();

    let n: Vec<usize> = iterator.next().unwrap().chars().map(|c| c.to_digit(10).unwrap() as usize).collect();

    let mut sum: usize = 0;
    for ni in n {
        sum += ni;
        sum %= 9;
    }
    if sum == 0 {
        "Yes".to_string()
    } else {
        "No".to_string()
    }
}

#[test]
fn test() {
    let cases: Vec<(&str, &str)> = vec![
        (
            r"123456789",
            "Yes"
        ),
        (
            r"0",
            "Yes"
        ),
        (
            r"31415926535897932384626433832795028841971693993751058209749445923078164062862089986280",
            "No"
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
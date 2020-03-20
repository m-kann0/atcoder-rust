use std::io::Read;

fn main() {
    let mut buf = String::new();
    std::io::stdin().read_to_string(&mut buf).unwrap();

    let answer = solve(&buf);

    println!("{}", answer);
}

fn solve(input: &str) -> String {
    let mut iterator = input.split_whitespace();

    let n = iterator.next().unwrap();

    let mut even_sum: usize = 0;
    let mut odd_sum: usize = 0;
    for (i, c) in n.chars().rev().enumerate() {
        if i % 2 == 0 {
            odd_sum += c.to_digit(10).unwrap() as usize;
        } else {
            even_sum += c.to_digit(10).unwrap() as usize;
        }
    }

    return format!("{} {}", even_sum, odd_sum);
}

#[test]
fn test() {
    let cases: Vec<(&str, &str)> = vec![
        (
            r"7392",
            "16 5"
        ),
        (
            r"12345",
            "6 9"
        ),
        (
            r"917237645269816381478124891628461341894621418946786785634501961",
            "142 163"
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
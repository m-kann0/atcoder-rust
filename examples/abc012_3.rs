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

    let mut correct = 0;
    for i in 1..=9 {
        for j in 1..=9 {
            correct += i * j;
        }
    }

    let diff = correct - n;

    let mut result = String::new();
    for i in 1..=9 {
        for j in 1..= 9 {
            if i * j == diff {
                result.push_str(&format!("{} x {}\n", i, j));
            }
        }
    }
    result.trim().to_string()
}

#[test]
fn test() {
    let cases: Vec<(&str, &str)> = vec![
        (
            r"2013",
            "2 x 6
3 x 4
4 x 3
6 x 2"
        ),
        (
            r"2024",
            "1 x 1"
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
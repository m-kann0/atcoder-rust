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
    let m: usize = iterator.next().unwrap().parse().unwrap();
    let k: usize = iterator.next().unwrap().parse().unwrap();

    for i in 0..(n + 1) {
        for j in 0..(m + 1) {
            let a = i * (m - j) + j * (n - i);
            if a == k {
                return String::from("Yes");
            }
        }
    }

    return String::from("No");
}

#[test]
fn test() {
    let cases: Vec<(&str, &str)> = vec![
        (
            r"2 2 2",
            "Yes"
        ),
        (
            r"2 2 1",
            "No"
        ),
        (
            r"3 5 8",
            "Yes"
        ),
        (
            r"7 9 20",
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
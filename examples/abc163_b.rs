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
    let mut a_sum: usize = 0;
    for _ in 0..m {
        let a: usize = iterator.next().unwrap().parse().unwrap();
        a_sum += a;
    }

    return if a_sum > n {
        "-1".to_string()
    } else {
        format!("{}", n - a_sum)
    };
}

#[test]
fn test() {
    let cases: Vec<(&str, &str)> = vec![
        (
            r"41 2
5 6",
            "30"
        ),
        (
            r"10 2
5 6",
            "-1"
        ),
        (
            r"11 2
5 6",
            "0"
        ),
        (
            r"314 15
9 26 5 35 8 9 79 3 23 8 46 2 6 43 3",
            "9"
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
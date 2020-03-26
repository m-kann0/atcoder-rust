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

    let mut c4: usize = 0;
    let mut c2: usize = 0;
    for _ in 0..n {
        let a: usize = iterator.next().unwrap().parse().unwrap();
        if a % 4 == 0 {
            c4 += 1;
        } else if a % 2 == 0 {
            c2 += 1;
        }
    }

    if c4 + c2 / 2 >= n / 2 {
        return String::from("Yes");
    }
    return String::from("No");
}

#[test]
fn test() {
    let cases: Vec<(&str, &str)> = vec![
        (
            r"3
1 10 100",
            r"Yes"
        ),
        (
            r"4
1 2 3 4",
            r"No"
        ),
        (
            r"3
1 4 1",
            r"Yes"
        ),
        (
            r"2
1 1",
            r"No"
        ),
        (
            r"6
2 7 1 8 2 8",
            r"Yes"
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
use std::io::Read;

fn main() {
    let mut buf = String::new();
    std::io::stdin().read_to_string(&mut buf).unwrap();

    let answer = solve(&buf);

    println!("{}", answer);
}

fn solve(input: &str) -> String {
    let mut iterator = input.split_whitespace();

    let n: isize = iterator.next().unwrap().parse().unwrap();
    let m: isize = iterator.next().unwrap().parse().unwrap();

    for x in 0..=n {
        let y = 4 * n - m - 2 * x;
        let z = m - 3 * n + x;
        if y >= 0 && z >= 0 {
            return format!("{} {} {}", x, y, z);
        }
    }

    "-1 -1 -1".to_string()
}

#[test]
fn test() {
    let cases: Vec<(&str, &str)> = vec![
        (
            r"3 9",
            r"1 1 1"
        ),
        (
            r"7 23",
            r"1 3 3"
        ),
        (
            r"10 41",
            r"-1 -1 -1"
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
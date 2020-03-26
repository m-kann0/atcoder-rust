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
    let total: usize = iterator.next().unwrap().parse().unwrap();

    for x in 0..(n + 1) {
        for y in 0..(n - x + 1) {
            let z = n - x - y;
            if 10000 * x + 5000 * y + 1000 * z == total {
                return format!("{} {} {}", x, y, z);
            }
        }
    }

    return String::from("-1 -1 -1");
}

#[test]
fn test() {
    let cases: Vec<(&str, &str)> = vec![
        (
            r"9 45000",
            "4 0 5"
        ),
        (
            r"20 196000",
            "-1 -1 -1"
        ),
        (
            r"1000 1234000",
            "14 27 959"
        ),
        (
            r"2000 20000000",
            "2000 0 0"
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
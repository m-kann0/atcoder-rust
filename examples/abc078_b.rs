use std::io::Read;

fn main() {
    let mut buf = String::new();
    std::io::stdin().read_to_string(&mut buf).unwrap();

    let answer = solve(&buf);

    println!("{}", answer);
}

fn solve(input: &str) -> String {
    let mut iterator = input.split_whitespace();

    let x: usize = iterator.next().unwrap().parse().unwrap();
    let y: usize = iterator.next().unwrap().parse().unwrap();
    let z: usize = iterator.next().unwrap().parse().unwrap();

    let mut ans = 0;
    for i in 0.. {
        if y * i + z * (i + 1) <= x {
            ans = i;
        } else {
            break;
        }
    }
    ans.to_string()
}

#[test]
fn test() {
    let cases: Vec<(&str, &str)> = vec![
        (
            r"13 3 1",
            "3"
        ),
        (
            r"12 3 1",
            "2"
        ),
        (
            r"100000 1 1",
            "49999"
        ),
        (
            r"64146 123 456",
            "110"
        ),
        (
            r"64145 123 456",
            "109"
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
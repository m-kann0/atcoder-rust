use std::io::Read;

fn main() {
    let mut buf = String::new();
    std::io::stdin().read_to_string(&mut buf).unwrap();

    let answer = solve(&buf);

    println!("{}", answer);
}

fn solve(input: &str) -> String {
    let mut iterator = input.split_whitespace();

    let mut x: isize = iterator.next().unwrap().parse().unwrap();
    let mut k: isize = iterator.next().unwrap().parse().unwrap();
    let d: isize = iterator.next().unwrap().parse().unwrap();

    if x < 0 {
        x = -x;
    }

    if k < x / d {
        return (x - d * k).abs().to_string();
    }

    k = k - x / d;
    x = x % d;

    if k % 2 == 0 {
        return x.abs().to_string();
    }

    return (x - d).abs().to_string();
}

#[test]
fn test() {
    let cases: Vec<(&str, &str)> = vec![
        (
            r"6 2 4",
            "2"
        ),
        (
            r"7 4 3",
            "1"
        ),
        (
            r"10 1 2",
            "8"
        ),
        (
            r"1000000000000000 1000000000000000 1000000000000000",
            "1000000000000000"
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
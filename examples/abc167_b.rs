use std::io::Read;

fn main() {
    let mut buf = String::new();
    std::io::stdin().read_to_string(&mut buf).unwrap();

    let answer = solve(&buf);

    println!("{}", answer);
}

fn solve(input: &str) -> String {
    let mut iterator = input.split_whitespace();

    let a: isize = iterator.next().unwrap().parse().unwrap();
    let b: isize = iterator.next().unwrap().parse().unwrap();
    let c: isize = iterator.next().unwrap().parse().unwrap();
    let mut k: isize = iterator.next().unwrap().parse().unwrap();

    if k < a {
        return k.to_string();
    }

    if a + b >= k {
        return a.to_string();
    }

    return (a - (k - a - b)).to_string();
}

#[test]
fn test() {
    let cases: Vec<(&str, &str)> = vec![
        (
            r"2 1 1 3",
            "2"
        ),
        (
            r"1 2 3 4",
            "0"
        ),
        (
            r"2000000000 0 0 2000000000",
            "2000000000"
        ),
        (
            r"3 1 1 1",
            "1"
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
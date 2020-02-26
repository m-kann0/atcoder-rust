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
    let mut vec: Vec<usize> = Vec::with_capacity(n);
    for _i in 0..n {
        vec.push(iterator.next().unwrap().parse().unwrap());
    }

    vec.sort();

    let a = vec[n / 2 - 1];
    let b = vec[n / 2];

    return if a == b {
        String::from("0")
    } else {
        format!("{}", b - a)
    };
}

#[test]
fn test() {
    let cases: Vec<(&str, &str)> = vec![
        (
            r"6
9 1 4 4 6 7",
            "2"
        ),
        (
            r"8
9 1 14 5 5 4 4 14",
            "0"
        ),
        (
            r"14
99592 10342 29105 78532 83018 11639 92015 77204 30914 21912 34519 80835 100000 1",
            "42685"
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
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

    let mut r: Vec<usize> = Vec::new();

    let mut prev: usize = 0;
    while r.len() < n {
        if prev == 0 {
            prev = 11;
            r.push(11);
        } else {
            let mut i = prev + 5;
            loop {
                if is_prime(i) {
                    prev = i;
                    r.push(i);
                    break;
                }
                i += 5;
            }
        }
    }

    return r.iter().map(|it| it.to_string()).collect::<Vec<String>>().join(" ");
}

fn is_prime(n: usize) -> bool {
    if n <= 1 {
        return false;
    }
    if n == 2 {
        return true;
    }
    if n % 2 == 0 {
        return false;
    }
    let mut i = 3;
    while i * i <= n {
        if n % i == 0 {
            return false;
        }
        i += 2;
    }
    return true;
}

#[test]
fn test() {
    let cases: Vec<(&str, &str)> = vec![
        (
            r"5",
            "3 5 7 11 31"
        ),
        (
            r"6",
            "2 3 5 7 11 13"
        ),
        (
            r"8",
            "2 5 7 13 19 37 67 79"
        ),
        (
            r"55",
            ""
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
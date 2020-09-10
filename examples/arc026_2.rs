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
    let s = sum_divisors(n);
    if s == n {
        "Perfect".to_string()
    } else if s < n {
        "Deficient".to_string()
    } else {
        "Abundant".to_string()
    }
}

fn sum_divisors(n: usize) -> usize {
    if n == 1 {
        return 0;
    }

    let mut result: usize = 0;

    let mut i = 1;
    while i * i <= n {
        if n % i == 0 {
            result += i;
            if i != 1 && n / i != i {
                result += n / i;
            }
        }
        if result > n {
            return result;
        }
        i += 1;
    }
    result
}

#[test]
fn test() {
    let cases: Vec<(&str, &str)> = vec![
        (
            r"6",
            "Perfect"
        ),
        (
            r"24",
            "Abundant"
        ),
        (
            r"27",
            "Deficient"
        ),
        (
            r"945",
            "Abundant"
        ),
        (
            r"10000000000",
            "Abundant"
        ),
        (
            r"1",
            "Deficient"
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
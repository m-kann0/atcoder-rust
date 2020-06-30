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

    let divisors = enumerate_divisors(m);

    let mut ans = 0;
    for d in divisors {
        if d > m / n {
            break;
        }
        if (m - (n - 1) * d) % d == 0 {
            ans = d;
        }
    }
    return ans.to_string();
}

fn enumerate_divisors(n: usize) -> Vec<usize> {
    let mut divisors = Vec::new();

    let mut i = 1;
    while i * i <= n {
        if n % i == 0 {
            divisors.push(i);
            if n / i != i {
                divisors.push(n / i);
            }
        }
        i += 1;
    }
    divisors.sort();
    return divisors;
}

#[test]
fn test() {
    let cases: Vec<(&str, &str)> = vec![
        (
            r"3 14",
            "2"
        ),
        (
            r"10 123",
            "3"
        ),
        (
            r"100000 1000000000",
            "10000"
        ),
        (
            r"1 11",
            "11"
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
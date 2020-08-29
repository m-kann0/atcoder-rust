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
    let a: Vec<usize> = (0..n).map(|_| iterator.next().unwrap().parse().unwrap()).collect();

    let mut is_coprime = true;
    let mut counts: Vec<usize> = vec![0; 1_000_005];
    for i in 0..n {
        let is_over = count_divisors(a[i], &mut counts);
        if is_over {
            is_coprime = false;
            break;
        }
    }

    if is_coprime {
        return "pairwise coprime".to_string();
    }

    let mut g = a[0];
    for i in 1..n {
        g = gcd(g, a[i]);
    }

    if g == 1 {
        "setwise coprime".to_string()
    } else {
        "not coprime".to_string()
    }
}

fn count_divisors(n: usize, counts: &mut Vec<usize>) -> bool {
    let mut i = 1;
    while i * i <= n {
        if n % i == 0 {
            counts[i] += 1;
            if i != 1 && counts[i] > 1 {
                return true;
            }
            if n / i != i {
                counts[n / i] += 1;
                if n / i != 1 && counts[n / i] > 1 {
                    return true;
                }
            }
        }
        i += 1;
    }
    return false;
}

fn gcd(mut x: usize, mut y: usize) -> usize {
    while y > 0 {
        let r = x % y;
        x = y;
        y = r;
    }
    return x;
}

#[test]
fn test() {
    let cases: Vec<(&str, &str)> = vec![
        (
            r"3
3 4 5",
            "pairwise coprime"
        ),
        (
            r"3
6 10 15",
            "setwise coprime"
        ),
        (
            r"3
6 10 16",
            "not coprime"
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
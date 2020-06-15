use std::io::Read;

fn main() {
    let mut buf = String::new();
    std::io::stdin().read_to_string(&mut buf).unwrap();

    let answer = solve(&buf);

    println!("{}", answer);
}

fn solve(input: &str) -> String {
    let mut iterator = input.split_whitespace();

    const MAX: usize = 100_000;
    let is_prime = make_sieve_of_eratosthenes(MAX);
    let mut s: Vec<usize> = vec![0; MAX + 1];
    let mut i = 3;
    while i <= MAX {
        if is_prime[i] && is_prime[(i + 1) / 2] {
            s[i] = s[i - 2] + 1;
        } else {
            s[i] = s[i - 2];
        }
        i += 2;
    }

    let q: usize = iterator.next().unwrap().parse().unwrap();
    let mut result = String::new();
    for _ in 0..q {
        let l: usize = iterator.next().unwrap().parse().unwrap();
        let r: usize = iterator.next().unwrap().parse().unwrap();

        let sl = if l == 1 {
            0
        } else {
            s[l - 2]
        };
        let sr = s[r];
        result.push_str(&format!("{}\n", sr - sl));
    }
    return result.trim().to_string();
}

fn make_sieve_of_eratosthenes(max: usize) -> Vec<bool> {
    let mut is_prime = vec![true; max + 1];
    is_prime[0] = false;
    is_prime[1] = false;

    let mut i = 2;
    while i * i <= max {
        if is_prime[i] {
            let mut j = i * 2;
            while j <= max {
                is_prime[j] = false;
                j += i;
            }
        }
        i += 1;
    }

    return is_prime;
}

#[test]
fn test() {
    let cases: Vec<(&str, &str)> = vec![
        (
            r"1
3 7",
            "2"
        ),
        (
            r"4
13 13
7 11
7 11
2017 2017",
            "1
0
0
1"
        ),
        (
            r"6
1 53
13 91
37 55
19 51
73 91
13 49",
            "4
4
1
1
1
2"
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
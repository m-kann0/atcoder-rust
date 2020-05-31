use std::io::Read;

fn main() {
    let mut buf = String::new();
    std::io::stdin().read_to_string(&mut buf).unwrap();

    let answer = solve(&buf);

    println!("{}", answer);
}

fn solve(input: &str) -> String {
    let mut iterator = input.split_whitespace();

    let mut wa: Vec<usize> = vec![0; 100001];
    for i in 1..100001 {
        if i % 2 == 0 {
            wa[i] = wa[i - 1];
        } else {
            if is_prime(i) && is_prime((i + 1) / 2) {
                wa[i] = wa[i - 1] + 1;
            } else {
                wa[i] = wa[i - 1];
            }
        }
    }

    let mut result = String::new();

    let q: usize = iterator.next().unwrap().parse().unwrap();
    for _ in 0..q {
        let l: usize = iterator.next().unwrap().parse().unwrap();
        let r: usize = iterator.next().unwrap().parse().unwrap();
        let ans = wa[r] - wa[l - 1];
        result.push_str(&format!("{}\n", ans));
    }

    return result.trim().to_string();
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
    let mut i: usize = 3;
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
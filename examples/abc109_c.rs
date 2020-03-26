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
    let x: isize = iterator.next().unwrap().parse().unwrap();

    let diffs: Vec<isize> = (0..n)
        .map(|_| (iterator.next().unwrap().parse::<isize>().unwrap() - x).abs())
        .collect::<Vec<isize>>();

    if diffs.len() == 1 {
        return diffs[0].to_string();
    }

    let mut ans: isize = gcd(diffs[0], diffs[1]);
    for i in 2..n {
        ans = gcd(ans, diffs[i]);
    }

    return ans.to_string();
}

fn gcd(m: isize, n: isize) -> isize {
    if n > m {
        gcd(n, m)
    } else if n == 0 {
        m
    } else {
        gcd(n, m % n)
    }
}

#[test]
fn test() {
    let cases: Vec<(&str, &str)> = vec![
        (
            r"3 3
1 7 11",
            "2"
        ),
        (
            r"3 81
33 105 57",
            "24"
        ),
        (
            r"1 1
1000000000",
            "999999999"
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
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
    let x: usize = iterator.next().unwrap().parse().unwrap();

    let mut a: Vec<usize> = vec![1; n + 1];
    let mut p: Vec<usize> = vec![1; n + 1];
    for i in 1..=n {
        a[i] = 2 * a[i - 1] + 3;
        p[i] = 2 * p[i - 1] + 1;
    }

    f(n, x, &a, &p).to_string()
}

fn f(n: usize, x: usize , a: &Vec<usize>, p: &Vec<usize>) -> usize {
    if n == 0 {
        return 1;
    }

    if x == 1 {
        return 0;
    }

    if x < a[n - 1] + 2 {
        return f(n - 1, x - 1, a, p);
    }

    if x == a[n - 1] + 2 {
        return p[n - 1] + 1;
    }

    if x < a[n] {
        return p[n - 1] + 1 + f(n - 1, x - (a[n - 1] + 2), a, p);
    }

    return p[n]
}

#[test]
fn test() {
    let cases: Vec<(&str, &str)> = vec![
        (
            r"2 7",
            "4"
        ),
        (
            r"1 1",
            "0"
        ),
        (
            r"50 4321098765432109",
            "2160549382716056"
        ),
        (
            r"3 1",
            "0"
        ),
        (
            r"3 2",
            "0"
        ),
        (
            r"3 3",
            "0"
        ),
        // NG
        (
            r"3 4",
            "1"
        ),
        (
            r"3 5",
            "2"
        ),
        (
            r"3 6",
            "3"
        ),
        (
            r"3 7",
            "3"
        ),
        (
            r"3 8",
            "4"
        ),
        (
            r"3 9",
            "4"
        ),
        (
            r"3 10",
            "5"
        ),
        (
            r"3 11",
            "6"
        ),
        (
            r"3 12",
            "7"
        ),
        (
            r"3 13",
            "7"
        ),
        (
            r"3 14",
            "7"
        ),
        (
            r"3 15",
            "8"
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
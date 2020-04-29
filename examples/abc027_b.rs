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
    let a: Vec<usize> = (0..n)
        .map(|_| iterator.next().unwrap().parse().unwrap())
        .collect();

    let total: usize = a.iter().fold(0, |acc, x| acc + *x);

    if total % n != 0 {
        return "-1".to_string();
    }

    let avg: isize = (total / n) as isize;

    let mut current = 0;
    let mut count = 0;
    for i in 0..n {
        if current != 0 {
            count += 1;
        }
        current += a[i] as isize - avg;
    }
    return count.to_string();
}

#[test]
fn test() {
    let cases: Vec<(&str, &str)> = vec![
        (
            r"3
1 2 3",
            "2"
        ),
        (
            r"5
2 0 0 0 3",
            "3"
        ),
        (
            r"2
0 99",
            "-1"
        ),
        (
            r"4
0 0 0 0",
            "0"
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
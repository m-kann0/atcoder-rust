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
    let mut l: Vec<usize> = (0..n)
        .map(|_| iterator.next().unwrap().parse().unwrap())
        .collect();

    l.sort();

    let mut count = 0;
    for i in 0..(n - 2) {
        for j in (i + 1)..(n - 1) {
            let a = l[i];
            let b = l[j];

            if !can_triangle(a, b, l[j + 1]) {
                continue;
            }

            if can_triangle(a, b, l[n - 1]) {
                count += n - 1 - j;
                continue;
            }

            let mut left = j + 1;
            let mut right = n - 1;
            while left + 1 < right {
                let mid = (right + left) / 2;
                if can_triangle(a, b, l[mid]) {
                    left = mid;
                } else {
                    right = mid;
                }
            }
            count += left - j;
        }
    }
    return count.to_string();
}

fn can_triangle(a: usize, b: usize, c: usize) -> bool {
    return a < b + c && b < c + a && c < a + b;
}

#[test]
fn test() {
    let cases: Vec<(&str, &str)> = vec![
        (
            r"4
3 4 2 1",
            "1"
        ),
        (
            r"3
1 1000 1",
            "0"
        ),
        (
            r"7
218 786 704 233 645 728 389",
            "23"
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
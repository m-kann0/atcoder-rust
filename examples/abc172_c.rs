use std::io::Read;
use std::cmp::max;

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
    let k: usize = iterator.next().unwrap().parse().unwrap();
    let a: Vec<usize> = (0..n).map(|_| iterator.next().unwrap().parse().unwrap()).collect();
    let b: Vec<usize> = (0..m).map(|_| iterator.next().unwrap().parse().unwrap()).collect();

    let mut ans: usize = 0;

    let mut count: usize = 0;
    let mut t: usize = k;
    let mut i: usize = 0;
    while i < n {
        if t >= a[i] {
            t -= a[i];
            count += 1;
            i += 1;
        } else {
            break;
        }
    }
    let mut j: usize = 0;
    while j < m {
        if t >= b[j] {
            t -= b[j];
            count += 1;
            j += 1;
        } else {
            break;
        }
    }
    ans = max(ans, count);

    while i > 0 {
        t += a[i - 1];
        count -= 1;
        i -= 1;
        while j < m {
            if t >= b[j] {
                t -= b[j];
                count += 1;
                j += 1;
            } else {
                break;
            }
        }
        ans = max(ans, count);
    }

    return ans.to_string();
}

#[test]
fn test() {
    let cases: Vec<(&str, &str)> = vec![
        (
            r"3 4 240
60 90 120
80 150 80 150",
            "3"
        ),
        (
            r"3 4 730
60 90 120
80 150 80 150",
            "7"
        ),
        (
            r"5 4 1
1000000000 1000000000 1000000000 1000000000 1000000000
1000000000 1000000000 1000000000 1000000000",
            "0"
        ),
        (
            r"3 4 30
10 10 1
20 1 1 1",
            "4"
        ),
        (
            r"4 3 30
20 1 1 1
10 10 1",
            "4"
        ),
        (
            r"3 4 1000000000
10 10 1
20 1 1 1",
            "7"
        ),
        (
            r"4 3 1000000000
20 1 1 1
10 10 1",
            "7"
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
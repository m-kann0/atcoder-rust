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
    let x: usize = iterator.next().unwrap().parse().unwrap();
    let y: usize = iterator.next().unwrap().parse().unwrap();
    let a: Vec<usize> = (0..n).map(|_| iterator.next().unwrap().parse().unwrap()).collect();
    let b: Vec<usize> = (0..m).map(|_| iterator.next().unwrap().parse().unwrap()).collect();

    let mut is_forward = true;
    let mut t: usize = 0;
    let mut l: usize = 0;
    let mut r: usize = 0;
    let mut count: usize = 0;
    while l < n && r < m {
        if is_forward {
            while l < n && a[l] < t {
                l += 1;
            }
            if l == n {
                break;
            }
            t = a[l] + x;
        } else {
            while r < m && b[r] < t {
                r += 1;
            }
            if r == m {
                break;
            }
            t = b[r] + y;
            count += 1;
        }
        is_forward = !is_forward;
    }
    return count.to_string();
}

#[test]
fn test() {
    let cases: Vec<(&str, &str)> = vec![
        (
            r"3 4
2 3
1 5 7
3 8 12 13",
            "2"
        ),
        (
            r"1 1
1 1
1
1",
            "0"
        ),
        (
            r"6 7
5 3
1 7 12 19 20 26
4 9 15 23 24 31 33",
            "3"
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
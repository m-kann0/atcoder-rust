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
    let mut a: Vec<usize> = (0..n)
        .map(|_| iterator.next().unwrap().parse().unwrap())
        .collect();
    a.sort();
    let mut b = a.clone();
    b.reverse();

    let mut ng: isize = -1;
    let mut ok: isize = n as isize;
    while (ok - ng).abs() > 1 {
        let mid = (ok + ng) / 2;
        let ai = a[mid as usize];
        let mut c = b.clone();
        c.remove(n - 1 - mid as usize);
        if can_survive(ai, &mut c) {
            ok = mid;
        } else {
            ng = mid;
        }
    }

    return format!("{}", n - ok as usize);
}

fn can_survive(mut size: usize, a: &mut Vec<usize>) -> bool {
    while !a.is_empty() {
        let size2: usize = a.pop().unwrap();
        if size2 <= 2 * size {
            size += size2;
        } else {
            return false;
        }
    }
    return true;
}

#[test]
fn test() {
    let cases: Vec<(&str, &str)> = vec![
        (
            r"3
3 1 4",
            "2"
        ),
        (
            r"5
1 1 1 1 1",
            "5"
        ),
        (
            r"6
40 1 30 2 7 20",
            "4"
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
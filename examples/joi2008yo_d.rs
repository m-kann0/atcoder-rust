#![allow(non_snake_case)]

use std::io::Read;
use std::collections::HashSet;

fn main() {
    let mut buf = String::new();
    std::io::stdin().read_to_string(&mut buf).unwrap();

    let answer = solve(&buf);

    println!("{}", answer);
}

fn solve(input: &str) -> String {
    let mut iterator = input.split_whitespace();

    let m: usize = iterator.next().unwrap().parse().unwrap();
    let mut a: Vec<(isize, isize)> = Vec::with_capacity(m);
    for _ in 0..m {
        let x: isize = iterator.next().unwrap().parse().unwrap();
        let y: isize = iterator.next().unwrap().parse().unwrap();
        a.push((x, y));
    }
    let n: usize = iterator.next().unwrap().parse().unwrap();
    let mut b: Vec<(isize, isize)> = Vec::with_capacity(n);
    let mut set: HashSet<(isize, isize)> = HashSet::new();
    for _ in 0..n {
        let x: isize = iterator.next().unwrap().parse().unwrap();
        let y: isize = iterator.next().unwrap().parse().unwrap();
        b.push((x, y));
        set.insert((x, y));
    }

    let origin = a[0];
    for i in 0..n {
        let dx = b[i].0 - origin.0;
        let dy = b[i].1 - origin.1;
        let mut is_ok = true;
        for j in 1..m {
            if !set.contains(&(a[j].0 + dx, a[j].1 + dy)) {
                is_ok = false;
                break;
            }
        }
        if is_ok {
            return format!("{} {}", dx, dy);
        }
    }

    unreachable!();
}

#[test]
fn test() {
    let cases: Vec<(&str, &str)> = vec![
        (
            r"5
8 5
6 4
4 3
7 10
0 10
10
10 5
2 7
9 7
8 10
10 2
1 2
8 1
6 7
6 0
0 9",
            "2 -3"
        ),
        (
            r"5
904207 809784
845370 244806
499091 59863
638406 182509
435076 362268
10
757559 866424
114810 239537
519926 989458
461089 424480
674361 448440
81851 150384
459107 795405
299682 6700
254125 362183
50795 541942",
            "-384281 179674"
        ),
    ];

    let mut all_ok = true;
    for (i, case) in cases.iter().enumerate() {
        print!("case {} : ", i);

        let expected = case.1;
        let actual = solve(case.0);

        if expected.trim() == actual.trim() {
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
#![allow(non_snake_case)]

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
    let mut a = Vec::new();
    let mut b = Vec::new();
    let mut c = Vec::new();
    let mut d = Vec::new();
    for _ in 0..n {
        let ai: isize = iterator.next().unwrap().parse().unwrap();
        let bi: isize = iterator.next().unwrap().parse().unwrap();
        a.push(ai);
        b.push(bi);
    }
    for _ in 0..m {
        let ci: isize = iterator.next().unwrap().parse().unwrap();
        let di: isize = iterator.next().unwrap().parse().unwrap();
        c.push(ci);
        d.push(di);
    }

    let mut result = String::new();
    for i in 0..n {
        let mut no: usize = 0;
        let mut diff: isize = std::isize::MAX;
        for j in 0..m {
            if (a[i] - c[j]).abs() + (b[i] - d[j]).abs() < diff {
                no = j + 1;
                diff = (a[i] - c[j]).abs() + (b[i] - d[j]).abs();
            }
        }
        result.push_str(&format!("{}\n", no));
    }
    result
}

#[test]
fn test() {
    let cases: Vec<(&str, &str)> = vec![
        (
            r"2 2
2 0
0 0
-1 0
1 0",
            "2
1"
        ),
        (
            r"3 4
10 10
-10 -10
3 3
1 2
2 3
3 5
3 5",
            "3
1
2"
        ),
        (
            r"5 5
-100000000 -100000000
-100000000 100000000
100000000 -100000000
100000000 100000000
0 0
0 0
100000000 100000000
100000000 -100000000
-100000000 100000000
-100000000 -100000000",
            "5
4
3
2
1"
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
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
    let mut a: Vec<usize> = (0..n).map(|_| iterator.next().unwrap().parse().unwrap()).collect();
    let mut b: Vec<usize> = (0..n).map(|_| iterator.next().unwrap().parse().unwrap()).collect();
    let mut c: Vec<usize> = (0..n).map(|_| iterator.next().unwrap().parse().unwrap()).collect();

    a.sort();
    b.sort();
    c.sort();

    let mut bc: Vec<usize> = vec![0; n];
    for i in 0..n {
        let bi = b[i];

        let mut ng: isize = -1;
        let mut ok: isize = n as isize;
        while (ok - ng).abs() > 1 {
            let mid = (ok + ng) / 2;
            if c[mid as usize] > bi {
                ok = mid;
            } else {
                ng = mid;
            }
        }
        if ok >= 0 && ok < n as isize {
            bc[i] = n - ok as usize;
        }
    }

    let mut bcs: Vec<usize> = vec![0; n];
    bcs[n - 1] = bc[n - 1];
    if n >= 2 {
        for i in (0..=(n - 2)).rev() {
            bcs[i] = bcs[i + 1] + bc[i];
        }
    }

    let mut ans: usize = 0;
    for i in 0..n {
        let ai = a[i];

        let mut ng: isize = -1;
        let mut ok: isize = n as isize;
        while (ok - ng).abs() > 1 {
            let mid = (ok + ng) / 2;
            if b[mid as usize] > ai {
                ok = mid;
            } else {
                ng = mid;
            }
        }
        if ok >= 0 && ok < n as isize {
            ans += bcs[ok as usize];
        }
    }
    ans.to_string()
}

#[test]
fn test() {
    let cases: Vec<(&str, &str)> = vec![
        (
            r"2
1 5
2 4
3 6",
            "3"
        ),
        (
            r"3
1 1 1
2 2 2
3 3 3",
            "27"
        ),
        (
            r"6
3 14 159 2 6 53
58 9 79 323 84 6
2643 383 2 79 50 288",
            "87"
        ),
        (
            r"1
1
2
3",
            "1"
        ),
        (
            r"1
3
2
1",
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
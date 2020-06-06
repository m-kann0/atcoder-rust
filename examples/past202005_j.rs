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

    let mut children: Vec<usize> = vec![0; n];
    let mut result = String::new();
    for _ in 0..m {
        let ai: usize = iterator.next().unwrap().parse().unwrap();

        let mut ng: isize = -1;
        let mut ok: isize = n as isize;
        while (ok - ng).abs() > 1 {
            let mid = (ok + ng) / 2;
            if children[mid as usize] < ai {
                ok = mid;
            } else {
                ng = mid;
            }
        }

        if ok < n as isize {
            result.push_str(&format!("{}\n", ok + 1));
            children[ok as usize] = ai;
        } else {
            result.push_str("-1\n");
        }
    }
    return result.trim().to_string();
}

#[test]
fn test() {
    let cases: Vec<(&str, &str)> = vec![
        (
            r"2 5
5 3 2 4 8",
            "1
2
-1
2
1"
        ),
        (
            r"5 10
13 16 6 15 10 18 13 17 11 3",
            "1
1
2
2
3
1
3
2
4
5"
        ),
        (
            r"10 30
35 23 43 33 38 25 22 39 22 6 41 1 15 41 3 20 50 17 25 14 1 22 5 10 34 38 1 12 15 1",
            "1
2
1
2
2
3
4
2
5
6
2
7
6
3
7
6
1
7
4
8
9
6
9
9
4
4
10
9
8
-1"
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
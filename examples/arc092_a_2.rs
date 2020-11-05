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
    let mut red: Vec<(usize, usize)> = Vec::new();
    let mut blue: Vec<(usize, usize)> = Vec::new();
    for _ in 0..n {
        let a: usize = iterator.next().unwrap().parse().unwrap();
        let b: usize = iterator.next().unwrap().parse().unwrap();
        red.push((a, b));
    }
    for _ in 0..n {
        let c: usize = iterator.next().unwrap().parse().unwrap();
        let d: usize = iterator.next().unwrap().parse().unwrap();
        blue.push((c, d));
    }

    blue.sort();

    let mut ans: usize = 0;
    let mut used = vec![false; n];
    for i in 0..n {
        let (bx, by) = blue[i];
        let mut candidates = Vec::new();
        for j in 0..n {
            if used[j] {
                continue;
            }
            let (ax, ay) = red[j];
            if ax < bx && ay < by {
                candidates.push(j);
            }
        }
        if candidates.is_empty() {
            continue;
        }
        candidates.sort_by_key(|it| red[*it].1);
        candidates.reverse();
        used[candidates[0]] = true;
        ans += 1;
    }
    ans.to_string()
}

#[test]
fn test() {
    let cases: Vec<(&str, &str)> = vec![
        (
            r"3
2 0
3 1
1 3
4 2
0 4
5 5",
            "2"
        ),
        (
            r"3
0 0
1 1
5 2
2 3
3 4
4 5",
            "2"
        ),
        (
            r"2
2 2
3 3
0 0
1 1",
            "0"
        ),
        (
            r"5
0 0
7 3
2 2
4 8
1 6
8 5
6 9
5 4
9 1
3 7",
            "5"
        ),
        (
            r"5
0 0
1 1
5 5
6 6
7 7
2 2
3 3
4 4
8 8
9 9",
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
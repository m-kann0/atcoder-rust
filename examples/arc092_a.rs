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
    let mut reds: Vec<(usize, usize)> =
        (0..n)
            .map(|_| {
                let ai: usize = iterator.next().unwrap().parse().unwrap();
                let bi: usize = iterator.next().unwrap().parse().unwrap();
                (ai, bi)
            })
            .collect();
    let mut blues: Vec<(usize, usize)> =
        (0..n)
            .map(|_| {
                let ci: usize = iterator.next().unwrap().parse().unwrap();
                let di: usize = iterator.next().unwrap().parse().unwrap();
                (ci, di)
            })
            .collect();

    reds.sort_by_key(|it| 1000 - it.1);
    blues.sort();

    let mut ans: usize = 0;
    let mut used: Vec<bool> = vec![false; n];
    for i in 0..n {
        for j in 0..n {
            if used[j] {
                continue;
            }
            if reds[j].0 < blues[i].0 && reds[j].1 < blues[i].1 {
                ans += 1;
                used[j] = true;
                break;
            }
        }
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
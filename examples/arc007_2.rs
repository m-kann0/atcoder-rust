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

    let mut cases: Vec<usize> = Vec::with_capacity(n + 1);
    for i in 0..(n + 1) {
        cases.push(i);
    }

    for _ in 0..m {
        let cd: usize = iterator.next().unwrap().parse().unwrap();
        let position = cases.iter().position(|it| *it == cd).unwrap();
        cases.swap(0, position);
    }

    let mut ans = String::new();
    for i in 1..(n + 1) {
        ans += &format!("{}\n", cases[i]);
    }
    return ans.trim().to_string();
}

#[test]
fn test() {
    let cases: Vec<(&str, &str)> = vec![
        (
            r"5 6
2
3
5
0
1
3",
            r"0
5
2
4
1",
        ),
        (
            r"3 5
0
1
1
1
2",
            r"0
1
3",
        ),
        (
            r"5 0",
            r"1
2
3
4
5",
        ),
        (
            r"10 7
2
8
5
3
3
8
1",
            r"8
0
5
4
3
6
7
2
9
10",
        ),
        (
            r"5 7
3
4
3
1
2
2
0",
            r"3
1
2
4
5",
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
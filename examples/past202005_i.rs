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
    let q: usize = iterator.next().unwrap().parse().unwrap();

    let mut cols: Vec<usize> = Vec::new();
    let mut rows: Vec<usize> = Vec::new();
    for i in 0..n {
        cols.push(i);
        rows.push(i);
    }

    let mut inversed = false;
    let mut result = String::new();
    for _ in 0..q {
        let query: usize = iterator.next().unwrap().parse().unwrap();
        match query {
            1 => {
                let a: usize = iterator.next().unwrap().parse().unwrap();
                let b: usize = iterator.next().unwrap().parse().unwrap();
                rows.swap(a - 1, b - 1);
            },
            2 => {
                let a: usize = iterator.next().unwrap().parse().unwrap();
                let b: usize = iterator.next().unwrap().parse().unwrap();
                cols.swap(a - 1, b - 1);
            },
            3 => {
                inversed = !inversed;
                let temp = cols;
                cols = rows;
                rows = temp;
            },
            _ => {
                let a: usize = iterator.next().unwrap().parse().unwrap();
                let b: usize = iterator.next().unwrap().parse().unwrap();
                let e: usize = if inversed {
                    rows[a - 1] + n * cols[b - 1]
                } else {
                    n * rows[a - 1] + cols[b - 1]
                };
                result.push_str(&format!("{}\n", e));
            },
        }
    }

    return result.trim().to_string();
}

#[test]
fn test() {
    let cases: Vec<(&str, &str)> = vec![
        (
            r"2
19
4 1 1
4 1 2
4 2 1
4 2 2
3
4 1 1
4 1 2
4 2 1
4 2 2
1 1 2
4 1 1
4 1 2
4 2 1
4 2 2
2 2 1
4 1 1
4 1 2
4 2 1
4 2 2",
            "0
1
2
3
0
2
1
3
1
3
0
2
3
1
2
0"
        ),
        (
            r"3
9
2 2 3
3
1 2 1
2 3 2
1 1 3
3
4 1 1
4 2 2
4 2 3",
            "1
6
8"
        ),
        (
            r"3
20
2 2 3
4 1 1
4 1 2
4 1 3
4 2 1
4 2 2
4 2 3
4 3 1
4 3 2
4 3 3
3
4 1 1
4 1 2
4 1 3
4 2 1
4 2 2
4 2 3
4 3 1
4 3 2
4 3 3",
            "0
2
1
3
5
4
6
8
7
0
3
6
2
5
8
1
4
7"
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
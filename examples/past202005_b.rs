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
    let q: usize = iterator.next().unwrap().parse().unwrap();

    let mut points: Vec<usize> = vec![n; m];
    let mut solved: Vec<Vec<bool>> = vec![vec![false; m]; n];

    let mut result = String::new();
    for _ in 0..q {
        let query: usize = iterator.next().unwrap().parse().unwrap();
        if query == 1 {
            let ni: usize = iterator.next().unwrap().parse().unwrap();
            let mut point: usize = 0;
            for i in 0..m {
                if solved[ni - 1][i] {
                    point += points[i];
                }
            }
            result.push_str(&format!("{}\n", point));
        } else {
            let ni: usize = iterator.next().unwrap().parse().unwrap();
            let mi: usize = iterator.next().unwrap().parse().unwrap();
            points[mi - 1] -= 1;
            solved[ni - 1][mi - 1] = true;
        }
    }

    return result.trim().to_string();
}

#[test]
fn test() {
    let cases: Vec<(&str, &str)> = vec![
        (
            r"2 1 6
2 1 1
1 1
1 2
2 2 1
1 1
1 2",
            "1
0
0
0"
        ),
        (
            r"5 5 30
1 3
2 3 5
1 3
2 2 1
2 4 5
2 5 2
2 2 3
1 4
2 4 1
2 2 2
1 1
1 5
2 5 3
2 4 4
1 4
1 2
2 3 3
2 4 3
1 3
1 5
1 3
2 1 3
1 1
2 2 4
1 1
1 4
1 5
1 4
1 1
1 5",
            "0
4
3
0
3
10
9
4
4
4
0
0
9
3
9
0
3"
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
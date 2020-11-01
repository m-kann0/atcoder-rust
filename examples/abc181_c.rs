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

    let mut points: Vec<(isize, isize)> = Vec::new();
    for _ in 0..n {
        let x: isize = iterator.next().unwrap().parse().unwrap();
        let y: isize = iterator.next().unwrap().parse().unwrap();
        points.push((x, y));
    }

    for i in 0..n {
        for j in 0..n {
            for k in 0..n {
                if i == j || j == k || k == i {
                    continue;
                }
                let p1 = points[i];
                let p2 = points[j];
                let p3 = points[k];
                if (p2.0 - p1.0) * (p3.1 - p1.1) == (p2.1 - p1.1) * (p3.0 - p1.0) {
                    return "Yes".to_string();
                }
            }
        }
    }
    "No".to_string()
}

#[test]
fn test() {
    let cases: Vec<(&str, &str)> = vec![
        (
            r"4
0 1
0 2
0 3
1 1",
            "Yes"
        ),
        (
            r"14
5 5
0 1
2 5
8 0
2 1
0 0
3 6
8 6
5 9
7 9
3 4
9 2
9 8
7 2",
            "No"
        ),
        (
            r"9
8 2
2 3
1 3
3 7
1 0
8 8
5 6
9 7
0 1",
            "Yes"
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
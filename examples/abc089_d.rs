use std::io::Read;

fn main() {
    let mut buf = String::new();
    std::io::stdin().read_to_string(&mut buf).unwrap();

    let answer = solve(&buf);

    println!("{}", answer);
}

fn solve(input: &str) -> String {
    let mut iterator = input.split_whitespace();

    let h: usize = iterator.next().unwrap().parse().unwrap();
    let w: usize = iterator.next().unwrap().parse().unwrap();
    let d: usize = iterator.next().unwrap().parse().unwrap();

    let mut points: Vec<(isize, isize)> = vec![(0, 0); h * w];
    for i in 0..h {
        for j in 0..w {
            let a: usize = iterator.next().unwrap().parse().unwrap();
            points[a - 1] = (i as isize, j as isize);
        }
    }

    let mut costs: Vec<usize> = vec![0; h * w];
    for i in 0..(h * w) {
        if i < d {
            continue;
        }
        costs[i] = costs[i - d] + (points[i].0 - points[i - d].0).abs() as usize + (points[i].1 - points[i - d].1).abs() as usize;
    }

    let mut result = String::new();
    let q: usize = iterator.next().unwrap().parse().unwrap();
    for _ in 0..q {
        let l: usize = iterator.next().unwrap().parse().unwrap();
        let r: usize = iterator.next().unwrap().parse().unwrap();
        let ans = costs[r - 1] - costs[l - 1];
        result.push_str(&format!("{}\n", ans));
    }
    result.trim().to_string()
}

#[test]
fn test() {
    let cases: Vec<(&str, &str)> = vec![
        (
            r"3 3 2
1 4 3
2 5 7
8 9 6
1
4 8",
            "5"
        ),
        (
            r"4 2 3
3 7
1 4
5 2
6 8
2
2 2
2 2",
            "0
0"
        ),
        (
            r"5 5 4
13 25 7 15 17
16 22 20 2 9
14 11 12 1 19
10 6 23 8 18
3 21 5 24 4
3
13 13
2 10
13 13",
            "0
5
0"
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
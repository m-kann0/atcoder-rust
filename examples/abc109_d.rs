use std::io::Read;
use std::collections::VecDeque;

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

    let mut map: Vec<Vec<usize>> = vec![vec![0; w]; h];
    for y in 0..h {
        for x in 0..w {
            let ai: usize = iterator.next().unwrap().parse().unwrap();
            map[y][x] = ai;
        }
    }

    let mut route = VecDeque::new();
    for y in 0..h {
        if y % 2 == 0 {
            for x in 0..w {
                route.push_back((y, x));
            }
        } else {
            for x in (0..w).rev() {
                route.push_back((y, x));
            }
        }
    }

    let mut logs = Vec::new();
    let mut current = route.pop_front().unwrap();
    while !route.is_empty() {
        let next = route.pop_front().unwrap();
        if map[current.0][current.1] % 2 == 1 {
            logs.push((current.0, current.1, next.0, next.1));
            map[current.0][current.1] -= 1;
            map[next.0][next.1] += 1;
        }
        current = next;
    }

    let mut result = String::new();
    result.push_str(&format!("{}\n", logs.len()));
    for log in logs {
        result.push_str(&format!("{} {} {} {}\n", log.0 + 1, log.1 + 1, log.2 + 1, log.3 + 1));
    }
    result.trim().to_string()
}

#[test]
fn test() {
    let cases: Vec<(&str, &str)> = vec![
        (
            r"2 3
1 2 3
0 1 1",
            "3
2 2 2 3
1 1 1 2
1 3 1 2"
        ),
        (
            r"3 2
1 0
2 1
1 0",
            "3
1 1 1 2
1 2 2 2
3 1 3 2"
        ),
        (
            r"1 5
9 9 9 9 9",
            "2
1 1 1 2
1 3 1 4"
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
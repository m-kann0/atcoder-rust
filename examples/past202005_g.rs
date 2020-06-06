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

    let n: usize = iterator.next().unwrap().parse().unwrap();
    let x: isize = iterator.next().unwrap().parse().unwrap();
    let y: isize = iterator.next().unwrap().parse().unwrap();

    let mut map: Vec<Vec<bool>> = vec![vec![true; 500]; 500];
    for _ in 0..n {
        let xi: isize = iterator.next().unwrap().parse().unwrap();
        let yi: isize = iterator.next().unwrap().parse().unwrap();
        map[(yi + 201) as usize][(xi + 201) as usize] = false;
    }

    let dx: Vec<isize> = vec![1, 0, -1, 1, -1, 0];
    let dy: Vec<isize> = vec![1, 1, 1, 0, 0, -1];

    const INF: usize = 1_000_000_000;
    let mut d: Vec<Vec<usize>> = vec![vec![INF; 500]; 500];
    let mut q: VecDeque<(usize, usize)> = VecDeque::new();
    d[201][201] = 0;
    q.push_back((201, 201));
    while !q.is_empty() {
        let current = q.pop_front().unwrap();
        let cy = current.0;
        let cx = current.1;

        for i in 0..6 {
            let ny = cy as isize + dy[i];
            let nx = cx as isize + dx[i];
            if nx < 0 || ny < 0 || nx > 410 || ny > 410 {
                continue;
            }
            let ny = ny as usize;
            let nx = nx as usize;
            if !map[ny][nx] || d[ny][nx] != INF {
                continue;
            }
            d[ny][nx] = d[cy][cx] + 1;
            q.push_back((ny, nx));
        }
    }

    let x: usize = (x + 201) as usize;
    let y: usize = (y + 201) as usize;
    if d[y][x] == INF {
        return "-1".to_string();
    }
    return format!("{}", d[y][x]);
}

#[test]
fn test() {
    let cases: Vec<(&str, &str)> = vec![
        (
            r"1 2 2
1 1",
            "3"
        ),
        (
            r"1 2 2
2 1",
            "2"
        ),
        (
            r"5 -2 3
1 1
-1 1
0 1
-2 1
-3 1",
            "6"
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
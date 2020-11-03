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
    let m: usize = iterator.next().unwrap().parse().unwrap();
    let mut map: Vec<Vec<char>> = Vec::new();
    for _ in 0..n {
        let line: Vec<char> = iterator.next().unwrap().chars().collect();
        map.push(line);
    }

    let mut ans: usize = 0;
    for i in 0..n {
        for j in 0..m {
            if map[i][j] == '.' {
                continue;
            }
            let mut map = map.clone();
            map[i][j] = '.';

            let dx = vec![1, -1, 0, 0];
            let dy = vec![0, 0, 1, -1];
            let mut q = VecDeque::new();
            q.push_back((i, j));
            map[i][j] = '#';
            while let Some((cx, cy)) = q.pop_front() {
                for k in 0..4 {
                    let nx = cx as isize + dx[k];
                    let ny = cy as isize + dy[k];
                    if nx < 0 || nx >= n as isize || ny < 0 || ny >= m as isize {
                        continue;
                    }
                    let nx = nx as usize;
                    let ny = ny as usize;
                    if map[nx][ny] == '.' {
                        map[nx][ny] = '#';
                        q.push_back((nx, ny));
                    }
                }
            }

            let mut is_ok = true;
            for p in 0..n {
                for q in 0..m {
                    if map[p][q] == '.' {
                        is_ok = false;
                    }
                }
            }
            if is_ok {
                ans += 1;
            }
        }
    }
    ans.to_string()
}

#[test]
fn test() {
    let cases: Vec<(&str, &str)> = vec![
        (
            r"3 3
..#
#..
.##",
            "2"
        ),
        (
            r"3 3
##.
##.
...",
            "3"
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
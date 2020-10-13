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
    let n: usize = iterator.next().unwrap().parse().unwrap();
    let mut map: Vec<Vec<char>> = Vec::new();
    for _ in 0..h {
        let line: Vec<char> = iterator.next().unwrap().chars().collect();
        map.push(line);
    }

    let mut start = (0, 0);
    let mut goals: Vec<(usize, usize)> = vec![(0, 0); n];
    for i in 0..h {
        for j in 0..w {
            if map[i][j] == 'S' {
                start = (i, j);
            }
            if let Some(x) = map[i][j].to_digit(10) {
                goals[x as usize - 1] = (i, j);
            }
        }
    }

    const INF: usize = 10_000_000;
    let dx: Vec<isize> = vec![1, -1, 0, 0];
    let dy: Vec<isize> = vec![0, 0, 1, -1];

    let mut ans: usize = 0;
    for i in 0..n {
        let goal = goals[i];

        let mut q: VecDeque<(usize, usize)> = VecDeque::new();
        let mut d: Vec<Vec<usize>> = vec![vec![INF; w]; h];

        q.push_back((start.0, start.1));
        d[start.0][start.1] = 0;
        while let Some((cx, cy)) = q.pop_front() {
            if cx == goal.0 && cy == goal.1 {
                break;
            }
            for j in 0..4 {
                let nx = cx as isize + dx[j];
                let ny = cy as isize + dy[j];
                if nx < 0 || nx >= h as isize || ny < 0 || ny >= w as isize {
                    continue;
                }
                let nx = nx as usize;
                let ny = ny as usize;
                if map[nx][ny] != 'X' && d[nx][ny] == INF {
                    q.push_back((nx, ny));
                    d[nx][ny] = d[cx][cy] + 1;
                }
            }
        }
        ans += d[goal.0][goal.1];

        start = goal;
    }
    ans.to_string()
}

#[test]
fn test() {
    let cases: Vec<(&str, &str)> = vec![
        (
            r"3 3 1
S..
...
..1",
            "4"
        ),
        (
            r"4 5 2
.X..1
....X
.XX.S
.2.X.",
            "12"
        ),
        (
            r"10 10 9
.X...X.S.X
6..5X..X1X
...XXXX..X
X..9X...X.
8.X2X..X3X
...XX.X4..
XX....7X..
X..X..XX..
X...X.XX..
..X.......",
            "91"
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
use std::io::Read;
use std::cmp::{min, max};
use std::collections::VecDeque;

fn main() {
    let mut buf = String::new();
    std::io::stdin().read_to_string(&mut buf).unwrap();

    let answer = solve(&buf);

    println!("{}", answer);
}

#[derive(Copy, Clone)]
struct Point {
    x: usize,
    y: usize,
}

impl Point {
    fn new(x: usize, y: usize) -> Self {
        Self { x, y }
    }
}

const INF: usize = 999;

fn solve(input: &str) -> String {
    let mut iterator = input.split_whitespace();

    let mut cards: Vec<Point> = Vec::new();
    let mut map: Vec<Vec<usize>> = vec![vec![INF; 20]; 20];
    for i in 0..100 {
        let x: usize = iterator.next().unwrap().parse().unwrap();
        let y: usize = iterator.next().unwrap().parse().unwrap();
        cards.push(Point::new(x, y));
        map[x][y] = i;
    }

    let mut result: Vec<char> = Vec::new();
    let mut current = Point::new(0, 0);
    for i in 0..100 {
        let card = cards[i];
        if let Some(p1) = exist(current, card, i + 1, &map) {
            if let Some(p2) = exist_empty(p1, card, &map) {
                let mut r1 = move_to(current, p1);
                result.append(&mut r1);
                result.push('I');
                map[p1.x][p1.y] = INF;
                let mut r2 = move_to(p1, p2);
                result.append(&mut r2);
                result.push('O');
                map[p2.x][p2.y] = i + 1;
                cards[i + 1] = p2;
                current = p2;
            }
        }

        let mut r = move_to(current, card);
        result.append(&mut r);
        result.push('I');
        current = card;
        map[current.x][current.y] = INF;
    }
    result.iter().collect::<String>()
}

fn exist(from: Point, to: Point, target: usize, map: &Vec<Vec<usize>>) -> Option<Point> {
    let minx = min(from.x, to.x);
    let maxx = max(from.x, to.x);
    let miny = min(from.y, to.y);
    let maxy = max(from.y, to.y);
    for x in minx..=maxx {
        for y in miny..=maxy {
            if map[x][y] == target {
                return Some(Point::new(x, y));
            }
        }
    }
    None
}

fn exist_empty(from: Point, to: Point, map: &Vec<Vec<usize>>) -> Option<Point> {
    let minx = min(from.x, to.x);
    let maxx = max(from.x, to.x);
    let miny = min(from.y, to.y);
    let maxy = max(from.y, to.y);

    let dx = vec![-1, 1, 0, 0];
    let dy = vec![0, 0, -1, 1];

    let mut q: VecDeque<Point> = VecDeque::new();
    let mut visited: Vec<Vec<bool>> = vec![vec![false; 20]; 20];
    q.push_back(to);
    visited[to.x][to.y] = true;
    while let Some(p) = q.pop_front() {
        if map[p.x][p.y] == INF {
            return Some(p);
        }
        for i in 0..4 {
            let nx = p.x as isize + dx[i];
            let ny = p.y as isize + dy[i];
            if nx < minx as isize || nx > maxx as isize || ny < miny as isize || ny > maxy as isize {
                continue;
            }
            let nx = nx as usize;
            let ny = ny as usize;
            if visited[nx][ny] {
                continue;
            }
            q.push_back(Point::new(nx, ny));
            visited[nx][ny] = true;
        }
    }
    None
}

fn move_to(from: Point, to: Point) -> Vec<char> {
    let mut result = Vec::new();
    if to.x > from.x {
        for _ in 0..(to.x - from.x) {
            result.push('D');
        }
    } else if to.x < from.x {
        for _ in 0..(from.x - to.x) {
            result.push('U');
        }
    }
    if to.y > from.y {
        for _ in 0..(to.y - from.y) {
            result.push('R');
        }
    } else if to.y < from.y {
        for _ in 0..(from.y - to.y) {
            result.push('L');
        }
    }
    return result;
}






#[test]
fn test_all() {
    let dir = "C:\\Users\\KannoMasato\\Desktop\\testCase\\";
    let mut total_score = 0;
    for i in 0..50 {
        let file_path = format!("{}testCase_{}.txt", dir, i);
        let input = std::fs::read_to_string(file_path).unwrap();

        let output = solve(&input);

        let score = evaluate(output);
        total_score += score;
    }
    println!("{}", total_score);
}

fn evaluate(output: String) -> isize {
    let mut score = 4000;
    let operations: Vec<char> = output.chars().collect();
    for i in 0..operations.len() {
        match operations[i] {
            'U' | 'D' | 'L' | 'R' => {
                score -= 1;
            },
            _ => {},
        }
    }
    score
}

#[test]
fn test_sample() {
    let input = r"15 19
8 1
3 13
2 19
17 10
14 3
3 2
19 4
6 2
18 1
4 4
3 10
0 15
2 5
10 7
6 3
19 12
1 0
19 3
4 1
0 6
10 18
12 12
8 13
6 4
10 2
6 12
2 0
0 11
6 9
8 3
13 9
9 0
11 17
9 4
12 1
1 18
19 19
9 9
2 11
8 19
18 3
2 15
8 16
16 2
4 5
14 4
9 3
15 13
3 0
8 11
15 4
0 7
12 19
18 7
12 17
8 2
6 0
1 7
17 0
16 10
1 6
10 5
4 14
12 7
6 11
6 6
14 19
15 15
17 16
1 12
7 9
2 7
4 7
1 4
5 3
0 17
17 3
5 13
9 16
12 13
11 12
15 18
0 10
19 6
12 4
10 8
15 1
10 13
17 15
1 5
15 14
19 5
17 7
2 18
5 7
12 6
3 6
19 7
1 16";
    println!("{}", evaluate(solve(input)));
}

#[test]
fn test() {
    let cases: Vec<(&str, &str)> = vec![
        (
            r"15 19
8 1
3 13
2 19
17 10
14 3
3 2
19 4
6 2
18 1
4 4
3 10
0 15
2 5
10 7
6 3
19 12
1 0
19 3
4 1
0 6
10 18
12 12
8 13
6 4
10 2
6 12
2 0
0 11
6 9
8 3
13 9
9 0
11 17
9 4
12 1
1 18
19 19
9 9
2 11
8 19
18 3
2 15
8 16
16 2
4 5
14 4
9 3
15 13
3 0
8 11
15 4
0 7
12 19
18 7
12 17
8 2
6 0
1 7
17 0
16 10
1 6
10 5
4 14
12 7
6 11
6 6
14 19
15 15
17 16
1 12
7 9
2 7
4 7
1 4
5 3
0 17
17 3
5 13
9 16
12 13
11 12
15 18
0 10
19 6
12 4
10 8
15 1
10 13
17 15
1 5
15 14
19 5
17 7
2 18
5 7
12 6
3 6
19 7
1 16",
            "DDDDDDDDDDDDDDDRRRRRRRRRRRRRRRRRRRIUUUUUUULLLLLLLLLLLLLLLLLLIUUUUURRRRRRRRRRRRIURRRRRRIDDDDDDDDDDDDDDDLLLLLLLLLIUUULLLLLLLIUUUUUUUUUUULIDDDDDDDDDDDDDDDDRRIUUUUUUUUUUUUULLIDDDDDDDDDDDDLIUUUUUUUUUUUUUURRRIURRRRRRIUUURRRRRIDDLLLLLLLLLLIDDDDDDDDRRIUUUULLLLIDDDDDDDDDDDDDRRRRRRRRRIUUUUUUUUUUUUUUUUUULLLLLLLLLLLLIDDDDDDDDDDDDDDDDDDRRRIUUUUUUUUUUUUUUULLIUUUURRRRRIDDDDDDDDDDRRRRRRRRRRRRIDDLLLLLLIUUUURIUULLLLLLLLLIDDDDLLIUUUURRRRRRRRRRIUUUULLLLLLLLLLLLIUURRRRRRRRRRRIDDDDDDLLIDDLLLLLLIDDDDDRRRRRRIUUUULLLLLLLLLIDDRRRRRRRRRRRRRRRRRIUULLLLLLLLLLLLLIDDDLLLIUUUUUUUUUUURRRRRRRRRRRRRRRRRIDDDDDDDDDDDDDDDDDDRIUUUUUUUUUULLLLLLLLLLIUUUUUUURRIDDDDDDRRRRRRRRIDDDDDDDDDDLLLLLLLLLLLLLLLLIUUUUUUUUUUUUUUUURRRRRRRRRRRRIDDDDDDRIDDDDDDDDLLLLLLLLLLLLLLIUUUUUUUUUUUURRRIDDDDDDDDDDLIUUUUULIDDDDDDRRRRRRRRRRIUUUUUUUUUUUULLLLLLLLLLLLLIDDDDDRRRRRRRRRRRIDDDDDDDLLLLLLLIUUUUUUUUUUUUUUURRRIDDDDDDDDDDDDRRRRRRRRRRRRIDDDDDDLLLLLLLLLLLLIUUUUUURRRRRRRRRRIUUUULLLLLLLLLLLLLLLIUULLIUUUUURRRRRRRIDDDDDDDDDDDDDDDDLLLLLLLIURRRRRRRRRRIUUUUUUUUUUUUUUULLLLIDDDDDDDDDLIUUUUUURRRRRRRRRIDDDDDDDDLLLLLLLIUUUUUURRRRILLLLLIDDDDDDDDRRRRRRRRRRRRRIDLLLLIDDRIUUUUUUUUUUUUUUUULLLLIDDDDDDLLLIUUUUULLIDDIUUULLLIDDDDLIUUUUURRRRRRRRRRRRRRIDDDDDDDDDDDDDDDDDLLLLLLLLLLLLLLIUUUUUUUUUUUURRRRRRRRRRIDDDDRRRIDDDLLLIULIDDDDRRRRRRIUUUUUUUUUUUUUUULLLLLLLLIDDDDDDDDDDDDDDDDDDDLLLLIUUUUUUULLIUURRRRIDDDDDLLLLLLLIUUUUURRRRRRRRRRRRIDDDDDDDRRIUUUUUUUUUUUUUUUULLLLLLLLLLIDDDDDDDDDDDDDDRRRRRRRRRIDDDDLLLLLLLLLIUURRIUUUUUUUUUUUUUUURRRRRRRRRRRIDDDLLLLLLLLLLLIDDDDDDDLIUUUUUUUUUIDDDDDDDDDDDDDDDDRIUUUUUUUUUUUUUUUUUURRRRRRRRRI"
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

use std::io::Read;

fn main() {
    let mut buf = String::new();
    std::io::stdin().read_to_string(&mut buf).unwrap();

    let answer = solve(&buf);

    println!("{}", answer);
}

const INF: usize = 100;

#[derive(Clone, Copy)]
struct Point {
    x: usize,
    y: usize,
}

impl Point {
    fn new(x: usize, y: usize) -> Self {
        Self { x, y }
    }
}

fn solve(input: &str) -> String {
    let mut iterator = input.split_whitespace();

    let mut map: Vec<Vec<usize>> = vec![vec![INF; 20]; 20];
    for i in 0..100 {
        let x: usize = iterator.next().unwrap().parse().unwrap();
        let y: usize = iterator.next().unwrap().parse().unwrap();
        map[x][y] = i;
    }

    let mut result: Vec<char> = Vec::new();
    let mut stack: Vec<usize> = Vec::new();
    let mut current = Point::new(0, 0);
    // 1. 回収
    {
        let mut points: Vec<Point> = Vec::new();
        for x in 0..20 {
            let y_limit = if x <= 9 {
                19
            } else {
                9
            };
            if x % 2 == 0 {
                for y in 0..=y_limit {
                    if map[x][y] != INF {
                        points.push(Point::new(x, y));
                    }
                }
            } else {
                for y in (0..=y_limit).rev() {
                    if map[x][y] != INF {
                        points.push(Point::new(x, y));
                    }
                }
            }
        }
        for &next in &points {
            let mut r = move_to(current, next);
            result.append(&mut r);
            current = next;
            stack.push(map[current.x][current.y]);
            map[current.x][current.y] = INF;
            result.push('I');
        }
    }
    // 2. 配置
    for x in (10..=19).rev() {
        if x % 2 == 1 {
            for y in 10..=19 {
                let next = Point::new(x, y);
                let mut r = move_to(current, next);
                result.append(&mut r);
                current = next;
                if !stack.is_empty() && map[current.x][current.y] == INF {
                    let i = stack.pop().unwrap();
                    map[current.x][current.y] = i;
                    result.push('O');
                }
            }
        } else {
            for y in (10..=19).rev() {
                let next = Point::new(x, y);
                let mut r = move_to(current, next);
                result.append(&mut r);
                current = next;
                if !stack.is_empty() && map[current.x][current.y] == INF {
                    let i = stack.pop().unwrap();
                    map[current.x][current.y] = i;
                    result.push('O');
                }
            }
        }
    }
    // 3. 回収
    let mut points: Vec<Point> = vec![Point::new(0, 0); 100];
    for x in 10..=19 {
        for y in 10..=19 {
            points[map[x][y]] = Point::new(x, y);
        }
    }
    for i in 0..100 {
        let next = points[i];
        let mut r = move_to(current, next);
        result.append(&mut r);
        current = next;
        result.push('I');
    }

    result.iter().collect::<String>()
}

fn move_to(from: Point, to: Point) -> Vec<char> {
    let mut v = Vec::new();
    if from.y < to.y {
        for _ in 0..(to.y - from.y) {
            v.push('R');
        }
    } else {
        for _ in 0..(from.y - to.y) {
            v.push('L');
        }
    }
    if from.x < to.x {
        for _ in 0..(to.x - from.x) {
            v.push('D');
        }
    } else {
        for _ in 0..(from.x - to.x) {
            v.push('U');
        }
    }
    v
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
    let output = solve(input);
    println!("{}", output);
    println!("{}", evaluate(output));
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
fn estimate() {
    // x軸方向の平均移動距離 * 2
    let mut bunshi: isize = 0;
    let mut bunbo: isize = 0;
    for cx in 0..=19_isize {
        for nx in 0..=19_isize {
            bunshi += (cx - nx).abs();
            bunbo += 1;
        }
    }
    println!("{}", bunshi as f64 / bunbo as f64 * 2.0);
}

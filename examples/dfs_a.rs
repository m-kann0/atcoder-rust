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
    let mut field: Vec<Vec<char>> = Vec::new();
    for _ in 0..h {
        let line: Vec<char> = iterator.next().unwrap().chars().collect();
        field.push(line);
    }

    for i in 0..h {
        for j in 0..w {
            if field[i][j] == 's' {
                return if dfs(h, w, &mut field, i, j) {
                    "Yes".to_string()
                } else {
                    "No".to_string()
                };
            }
        }
    }

    return String::new();
}

fn dfs(h: usize, w: usize, field: &mut Vec<Vec<char>>, cx: usize, cy: usize) -> bool {
    if field[cx][cy] == 'g' {
        return true;
    }

    field[cx][cy] = '#';

    let dx = vec![-1, 1, 0, 0];
    let dy = vec![0, 0, -1, 1];
    for i in 0..4 {
        let nx = cx as isize + dx[i];
        let ny = cy as isize + dy[i];
        if nx < 0 || nx >= h as isize {
            continue;
        }
        if ny < 0 || ny >= w as isize {
            continue;
        }
        let nx = nx as usize;
        let ny = ny as usize;
        if field[nx][ny] == '#' {
            continue;
        }
        if dfs(h, w, field, nx, ny) {
            return true;
        }
    }
    false
}

#[test]
fn test() {
    let cases: Vec<(&str, &str)> = vec![
        (
            r"4 5
s####
....#
#####
#...g",
            r"No",
        ),
        (
            r"4 4
...s
....
....
.g..",
            r"Yes",
        ),
        (
            r"10 10
s.........
#########.
#.......#.
#..####.#.
##....#.#.
#####.#.#.
g.#.#.#.#.
#.#.#.#.#.
###.#.#.#.
#.....#...",
            r"No",
        ),
        (
            r"10 10
s.........
#########.
#.......#.
#..####.#.
##....#.#.
#####.#.#.
g.#.#.#.#.
#.#.#.#.#.
#.#.#.#.#.
#.....#...",
            r"Yes",
        ),
        (
            r"1 10
s..####..g",
            r"No",
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
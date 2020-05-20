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
    let mut map: Vec<Vec<char>> = Vec::new();
    for _ in 0..h {
        map.push(iterator.next().unwrap().chars().collect());
    }

    let mut count: usize = 0;
    let mut visited = vec![vec![false; w]; h];
    for y in 0..h {
        for x in 0..w {
            if visited[y][x] {
                continue;
            }

            let mut black: usize = 0;
            let mut white: usize = 0;

            let mut stack: Vec<(usize, usize)> = Vec::new();
            stack.push((y, x));
            visited[y][x] = true;
            if map[y][x] == '.' {
                white += 1;
            } else {
                black += 1;
            }
            while !stack.is_empty() {
                let top = *stack.last().unwrap();
                if top.1 > 0 && !visited[top.0][top.1 - 1] && map[top.0][top.1 - 1] != map[top.0][top.1] {
                    stack.push((top.0, top.1 - 1));
                    visited[top.0][top.1 - 1] = true;
                    if map[top.0][top.1 - 1] == '.' {
                        white += 1;
                    } else {
                        black += 1;
                    }
                } else if top.0 > 0 && !visited[top.0 - 1][top.1] && map[top.0 - 1][top.1] != map[top.0][top.1] {
                    stack.push((top.0 - 1, top.1));
                    visited[top.0 - 1][top.1] = true;
                    if map[top.0 - 1][top.1] == '.' {
                        white += 1;
                    } else {
                        black += 1;
                    }
                } else if top.1 + 1 < w && !visited[top.0][top.1 + 1] && map[top.0][top.1 + 1] != map[top.0][top.1] {
                    stack.push((top.0, top.1 + 1));
                    visited[top.0][top.1 + 1] = true;
                    if map[top.0][top.1 + 1] == '.' {
                        white += 1;
                    } else {
                        black += 1;
                    }
                } else if top.0 + 1 < h && !visited[top.0 + 1][top.1] && map[top.0 + 1][top.1] != map[top.0][top.1] {
                    stack.push((top.0 + 1, top.1));
                    visited[top.0 + 1][top.1] = true;
                    if map[top.0 + 1][top.1] == '.' {
                        white += 1;
                    } else {
                        black += 1;
                    }
                } else {
                    stack.pop();
                }
            }
            count += white * black;
        }
    }

    return count.to_string();
}

#[test]
fn test() {
    let cases: Vec<(&str, &str)> = vec![
        (
            r"3 3
.#.
..#
#..",
            "10"
        ),
        (
            r"2 4
....
....",
            "0"
        ),
        (
            r"4 3
###
###
...
###",
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
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
        let line: Vec<char> = iterator.next().unwrap().chars().collect();
        map.push(line);
    }

    for r in (0..h).rev() {
        let mut is_all_white = true;
        for c in 0..w {
            if map[r][c] == '#' {
                is_all_white = false;
                break;
            }
        }
        if is_all_white {
            map.remove(r);
        }
    }

    for c in (0..w).rev() {
        let mut is_all_white = true;
        for r in 0..map.len() {
            if map[r][c] == '#' {
                is_all_white = false;
                break;
            }
        }
        if is_all_white {
            for r in 0..map.len() {
                map[r].remove(c);
            }
        }
    }

    let mut result = String::new();
    for line in &map {
        for &c in line {
            result.push(c);
        }
        result.push('\n');
    }
    return result.trim().to_string();
}

#[test]
fn test() {
    let cases: Vec<(&str, &str)> = vec![
        (
            r"4 4
##.#
....
##.#
.#.#",
            "###
###
.##"
        ),
        (
            r"3 3
#..
.#.
..#",
            "#..
.#.
..#"
        ),
        (
            r"4 5
.....
.....
..#..
.....",
            "#"
        ),
        (
            r"7 6
......
....#.
.#....
..#...
..#...
......
.#..#.",
            "..#
#..
.#.
.#.
#.#"
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
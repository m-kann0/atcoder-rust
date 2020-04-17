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
    let mut map: Vec<Vec<char>> = vec![vec!['.'; w + 2]; h + 2];
    for i in 0..h {
        let line: Vec<char> = iterator.next().unwrap().chars().collect();
        for j in 0..w {
            map[i + 1][j + 1] = line[j];
        }
    }

    let mut possible: bool = true;
    let mut x = 1;
    let mut y = 1;
    let mut d = Direction::RIGHT;
    while !(x == w && y == h) {
        if d == Direction::RIGHT && map[y - 1][x] == '#' {
            possible = false;
            break;
        }
        if d == Direction::DOWN && map[y][x - 1] == '#' {
            possible = false;
            break;
        }

        if map[y][x + 1] == '#' && map[y + 1][x] == '#' {
            possible = false;
            break;
        }
        if map[y][x + 1] == '#' {
            d = Direction::RIGHT;
            x += 1;
            continue;
        }
        if map[y + 1][x] == '#' {
            d = Direction::DOWN;
            y += 1;
            continue;
        }
        possible = false;
        break;
    }

    if d == Direction::RIGHT && map[y - 1][x] == '#' {
        possible = false;
    }
    if d == Direction::DOWN && map[y][x - 1] == '#' {
        possible = false;
    }

    return if possible {
        String::from("Possible")
    } else {
        String::from("Impossible")
    };
}

#[derive(PartialEq)]
enum Direction {
    RIGHT, DOWN,
}

#[test]
fn test() {
    let cases: Vec<(&str, &str)> = vec![
        (
            r"4 5
##...
.##..
..##.
...##",
            "Possible"
        ),
        (
            r"5 3
###
..#
###
#..
###",
            "Impossible"
        ),
        (
            r"4 5
##...
.###.
.###.
...##",
            "Impossible"
        ),
        (
            r"4 5
#####
....#
....#
...##",
            "Impossible"
        ),
        (
            r"4 5
#....
#....
#...#
#####",
            "Impossible"
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
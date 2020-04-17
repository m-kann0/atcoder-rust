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
    for i in 1..(h + 1) {
        for j in 1..(w + 1) {
            if map[i][j] != '#' {
                continue;
            }
            if i == 1 && j == 1 {
                if map[i][j + 1] == '#' && map[i + 1][j] == '#' {
                    possible = false;
                    break;
                } else {
                    continue;
                }
            }
            if i == h && j == w {
                if map[i][j - 1] == '#' && map[i - 1][j] == '#' {
                    possible = false;
                    break;
                } else {
                    continue;
                }
            }

            if map[i][j + 1] == '#' && map[i + 1][j] == '#' {
                possible = false;
                break;
            }
            if map[i][j - 1] == '#' && map[i - 1][j] == '#' {
                possible = false;
                break;
            }
        }
    }

    return if possible {
        String::from("Possible")
    } else {
        String::from("Impossible")
    };
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
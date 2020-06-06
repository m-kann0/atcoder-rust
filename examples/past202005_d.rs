use std::io::Read;

fn main() {
    let mut buf = String::new();
    std::io::stdin().read_to_string(&mut buf).unwrap();

    let answer = solve(&buf);

    println!("{}", answer);
}

fn solve(input: &str) -> String {
    let mut iterator = input.split_whitespace();

    let n: usize = iterator.next().unwrap().parse().unwrap();

    let numbers: Vec<Vec<Vec<char>>> = init_numbers();

    let mut board: Vec<Vec<char>> = Vec::new();
    for _ in 0..5 {
        board.push(iterator.next().unwrap().chars().collect());
    }

    let mut result: String = String::new();
    for i in 0..n {
        for j in 0..10 {
            let mut correct = true;
            for k in 0..4 {
                for m in 0..5 {
                    if board[m][4 * i + k] != numbers[j][m][k] {
                        correct = false;
                    }
                }
            }
            if correct {
                result.push_str(&format!("{}", j));
                break;
            }
        }
    }

    return result;
}

fn init_numbers() -> Vec<Vec<Vec<char>>> {
    return vec![
        vec![
            ".###".chars().collect(),
            ".#.#".chars().collect(),
            ".#.#".chars().collect(),
            ".#.#".chars().collect(),
            ".###".chars().collect(),
        ],
        vec![
            "..#.".chars().collect(),
            ".##.".chars().collect(),
            "..#.".chars().collect(),
            "..#.".chars().collect(),
            ".###".chars().collect(),
        ],
        vec![
            ".###".chars().collect(),
            "...#".chars().collect(),
            ".###".chars().collect(),
            ".#..".chars().collect(),
            ".###".chars().collect(),
        ],
        vec![
            ".###".chars().collect(),
            "...#".chars().collect(),
            ".###".chars().collect(),
            "...#".chars().collect(),
            ".###".chars().collect(),
        ],
        vec![
            ".#.#".chars().collect(),
            ".#.#".chars().collect(),
            ".###".chars().collect(),
            "...#".chars().collect(),
            "...#".chars().collect(),
        ],
        vec![
            ".###".chars().collect(),
            ".#..".chars().collect(),
            ".###".chars().collect(),
            "...#".chars().collect(),
            ".###".chars().collect(),
        ],
        vec![
            ".###".chars().collect(),
            ".#..".chars().collect(),
            ".###".chars().collect(),
            ".#.#".chars().collect(),
            ".###".chars().collect(),
        ],
        vec![
            ".###".chars().collect(),
            "...#".chars().collect(),
            "...#".chars().collect(),
            "...#".chars().collect(),
            "...#".chars().collect(),
        ],
        vec![
            ".###".chars().collect(),
            ".#.#".chars().collect(),
            ".###".chars().collect(),
            ".#.#".chars().collect(),
            ".###".chars().collect(),
        ],
        vec![
            ".###".chars().collect(),
            ".#.#".chars().collect(),
            ".###".chars().collect(),
            "...#".chars().collect(),
            ".###".chars().collect(),
        ],
    ];
}

#[test]
fn test() {
    let cases: Vec<(&str, &str)> = vec![
        (
            r"10
.###..#..###.###.#.#.###.###.###.###.###.
.#.#.##....#...#.#.#.#...#.....#.#.#.#.#.
.#.#..#..###.###.###.###.###...#.###.###.
.#.#..#..#.....#...#...#.#.#...#.#.#...#.
.###.###.###.###...#.###.###...#.###.###.",
            "0123456789"
        ),
        (
            r"29
.###.###.###.###.###.###.###.###.###.#.#.###.#.#.#.#.#.#.###.###.###.###..#..###.###.###.###.###.#.#.###.###.###.###.
...#.#.#...#.#.#.#.#.#...#.#...#.#.#.#.#.#...#.#.#.#.#.#.#.....#.#.#.#.#.##..#.#...#.#.#...#.#...#.#...#.#.....#...#.
.###.#.#...#.###.#.#.###.###...#.###.###.###.###.###.###.###...#.###.#.#..#..###...#.###.###.###.###.###.###.###.###.
.#...#.#...#...#.#.#.#.#...#...#.#.#...#.#.#...#...#...#.#.#...#...#.#.#..#..#.#...#...#.#...#.#...#.#.....#...#.#...
.###.###...#.###.###.###.###...#.###...#.###...#...#...#.###...#.###.###.###.###...#.###.###.###...#.###.###.###.###.",
            "20790697846444679018792642532"
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
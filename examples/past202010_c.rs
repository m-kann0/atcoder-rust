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
    let m: usize = iterator.next().unwrap().parse().unwrap();
    let mut map: Vec<Vec<char>> = Vec::new();
    for _ in 0..n {
        let line: Vec<char> = iterator.next().unwrap().chars().collect();
        map.push(line);
    }

    let mut result: Vec<Vec<usize>> = vec![vec![0; m]; n];
    for i in 0..n {
        for j in 0..m {
            for di in -1..=1 {
                let ci = i as isize + di;
                if ci < 0 || ci >= n as isize {
                    continue;
                }
                let ci = ci as usize;
                for dj in -1..=1 {
                    let cj = j as isize + dj;
                    if cj < 0 || cj >= m as isize {
                        continue;
                    }
                    let cj = cj as usize;
                    if map[ci][cj] == '#' {
                        result[i][j] += 1;
                    }
                }
            }
        }
    }

    let mut ans = String::new();
    for i in 0..n {
        let mut line = String::new();
        for j in 0..m {
            line.push_str(&format!("{}", result[i][j]));
        }
        line.push('\n');
        ans.push_str(&line);
    }
    ans.trim().to_string()
}

#[test]
fn test() {
    let cases: Vec<(&str, &str)> = vec![
        (
            r"3 4
#.##
..#.
#...",
            "1333
2433
1211"
        ),
        (
            r"10 12
#.##..#...##
#..#..##...#
##.#....##.#
.#..###...#.
#..#..#...##
###...#..###
.###.#######
.#..#....###
.#.##..####.
.###....#..#",
            "233322331133
455432343354
444344443343
444344332454
454335431465
466434554686
466434445796
346554457885
346542135664
235431134432"
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
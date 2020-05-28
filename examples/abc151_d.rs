use std::io::Read;
use std::collections::VecDeque;
use std::cmp::max;

fn main() {
    let mut buf = String::new();
    std::io::stdin().read_to_string(&mut buf).unwrap();

    let answer = solve(&buf);

    println!("{}", answer);
}

fn solve(input: &str) -> String {
    let mut iterator = input.split_whitespace();

    let h: isize = iterator.next().unwrap().parse().unwrap();
    let w: isize = iterator.next().unwrap().parse().unwrap();
    let mut map: Vec<Vec<char>> = Vec::new();
    for _ in 0..h {
        let line: Vec<char> = iterator.next().unwrap().chars().collect();
        map.push(line);
    }

    let di: Vec<isize> = vec![-1, 1, 0, 0];
    let dj: Vec<isize> = vec![0, 0, -1, 1];

    let mut ans: isize = 0;
    for i in 0..h {
        for j in 0..w {
            if map[i as usize][j as usize] == '#' {
                continue;
            }

            let mut d: Vec<Vec<isize>> = vec![vec![-1; w as usize]; h as usize];
            let mut q: VecDeque<(isize, isize)> = VecDeque::new();
            d[i as usize][j as usize] = 0;
            q.push_back((i, j));
            let mut dmax = 0;
            while !q.is_empty() {
                let (ci, cj) = q.pop_front().unwrap();
                for k in 0..4 {
                    let ni = ci + di[k];
                    let nj = cj + dj[k];
                    if ni < 0 || ni >= h || nj < 0 || nj >= w {
                        continue;
                    }
                    if map[ni as usize][nj as usize] == '#' {
                        continue;
                    }
                    if d[ni as usize][nj as usize] != -1 {
                        continue;
                    }
                    d[ni as usize][nj as usize] = d[ci as usize][cj as usize] + 1;
                    dmax = max(dmax, d[ni as usize][nj as usize]);
                    q.push_back((ni, nj));
                }
            }
            ans = max(ans, dmax);
        }
    }

    return ans.to_string();
}

#[test]
fn test() {
    let cases: Vec<(&str, &str)> = vec![
        (
            r"3 3
...
...
...",
            "4"
        ),
        (
            r"3 5
...#.
.#.#.
.#...",
            "10"
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
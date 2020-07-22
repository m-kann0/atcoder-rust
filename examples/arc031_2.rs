use std::io::Read;
use std::collections::VecDeque;

fn main() {
    let mut buf = String::new();
    std::io::stdin().read_to_string(&mut buf).unwrap();

    let answer = solve(&buf);

    println!("{}", answer);
}

fn solve(input: &str) -> String {
    let mut iterator = input.split_whitespace();

    let mut map: Vec<Vec<char>> = Vec::new();
    for _ in 0..10 {
        let line: Vec<char> = iterator.next().unwrap().chars().collect();
        map.push(line);
    }

    let mut start: (usize, usize) = (0, 0);
    for i in 0..10 {
        let mut found = false;
        for j in 0..10 {
            if map[i][j] == 'o' {
                start = (i, j);
                found = true;
                break;
            }
        }
        if found {
            break;
        }
    }

    let mut count_o: usize = 0;
    for i in 0..10 {
        for j in 0..10 {
            if map[i][j] == 'o' {
                count_o += 1;
            }
        }
    }

    let di: Vec<isize> = vec![-1, 1, 0, 0];
    let dj: Vec<isize> = vec![0, 0, -1, 1];

    for i in 0..10 {
        for j in 0..10 {
            if map[i][j] == 'o' {
                continue;
            }

            let mut current_map = map.clone();
            current_map[i][j] = 'o';

            let mut q: VecDeque<(usize, usize)> = VecDeque::new();
            q.push_back(start);
            let mut visited: Vec<Vec<bool>> = vec![vec![false; 10]; 10];
            visited[start.0][start.1] = true;
            let mut count = 1;
            while !q.is_empty() {
                let (ci, cj) = q.pop_front().unwrap();
                for k in 0..4 {
                    let ni: isize = ci as isize + di[k];
                    let nj: isize = cj as isize + dj[k];
                    if ni < 0 || ni > 9 || nj < 0 || nj > 9 {
                        continue;
                    }
                    let ni = ni as usize;
                    let nj = nj as usize;
                    if current_map[ni][nj] == 'o' && !visited[ni][nj] {
                        q.push_back((ni, nj));
                        visited[ni][nj] = true;
                        count += 1;
                    }
                }
            }
            if count_o + 1 == count {
                return "YES".to_string();
            }
        }
    }

    "NO".to_string()
}

#[test]
fn test() {
    let cases: Vec<(&str, &str)> = vec![
        (
            r"xxxxxxxxxx
xoooooooxx
xxoooooxxx
xxxoooxxxx
xxxxoxxxxx
xxxxxxxxxx
xxxxoxxxxx
xxxoooxxxx
xxoooooxxx
xxxxxxxxxx",
            "YES"
        ),
        (
            r"xxxxxxxxxx
xoooooooxx
xxoooooxxx
xxxoooxxxx
xxxxxxxxxx
xxxxxxxxxx
xxxxxxxxxx
xxxoooxxxx
xxoooooxxx
xxxxxxxxxx",
            "NO"
        ),
        (
            r"xxxxoxxxxx
xxxxoxxxxx
xxxxoxxxxx
xxxxoxxxxx
ooooxooooo
xxxxoxxxxx
xxxxoxxxxx
xxxxoxxxxx
xxxxoxxxxx
xxxxoxxxxx",
            "YES"
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
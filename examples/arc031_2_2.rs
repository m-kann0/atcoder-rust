use std::io::Read;

fn main() {
    let mut buf = String::new();
    std::io::stdin().read_to_string(&mut buf).unwrap();

    let answer = solve(&buf);

    println!("{}", answer);
}

fn solve(input: &str) -> String {
    let mut iterator = input.split_whitespace();

    let mut field: Vec<Vec<char>> = Vec::new();
    for _ in 0..10 {
        let line: Vec<char> = iterator.next().unwrap().chars().collect();
        field.push(line);
    }

    for i in 0..10 {
        for j in 0..10 {
            let mut field = field.clone();
            field[i][j] = 'o';

            dfs(&mut field, i, j);

            if all_x(&field) {
                return "YES".to_string();
            }
        }
    }

    return "NO".to_string()
}

fn dfs(field: &mut Vec<Vec<char>>, cx: usize, cy: usize) {
    field[cx][cy] = 'x';

    let dx = vec![-1, 1, 0, 0];
    let dy = vec![0, 0, -1, 1];
    for i in 0..4 {
        let nx = cx as isize + dx[i];
        let ny = cy as isize + dy[i];
        if nx < 0 || nx >= 10 || ny < 0 || ny >= 10 {
            continue;
        }
        let nx = nx as usize;
        let ny = ny as usize;
        if field[nx][ny] == 'o' {
            dfs(field, nx, ny);
        }
    }
}

fn all_x(field: &Vec<Vec<char>>) -> bool {
    for i in 0..10 {
        for j in 0..10 {
            if field[i][j] != 'x' {
                return false;
            }
        }
    }
    true
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
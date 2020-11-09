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
    let n: usize = iterator.next().unwrap().parse().unwrap();
    let m: usize = iterator.next().unwrap().parse().unwrap();

    let mut lamps: Vec<(usize, usize)> = vec![(0, 0); n];
    let mut map: Vec<Vec<usize>> = vec![vec![0; w]; h];
    const LAMP: usize = 1;
    const BLOCK: usize = 2;
    for i in 0..n {
        let a: usize = iterator.next().unwrap().parse().unwrap();
        let b: usize = iterator.next().unwrap().parse().unwrap();
        let a = a - 1;
        let b = b - 1;
        lamps[i] = (a, b);
        map[a][b] = LAMP;
    }
    for _ in 0..m {
        let c: usize = iterator.next().unwrap().parse().unwrap();
        let d: usize = iterator.next().unwrap().parse().unwrap();
        let c = c - 1;
        let d = d - 1;
        map[c][d] = BLOCK;
    }

    let dh = vec![-1, 0, 1, 0];
    let dw = vec![0, -1, 0, 1];

    let mut ans: usize = 0;
    let mut lighted: Vec<Vec<bool>> = vec![vec![false; w]; h];
    let mut has_lamp: Vec<Vec<Vec<bool>>> = vec![vec![vec![false; 4]; w]; h];
    for &(a, b) in &lamps {
        for i in 0..4 {
            if has_lamp[a][b][i] {
                continue;
            }
            let dh = dh[i];
            let dw = dw[i];
            let mut ch = a as isize;
            let mut cw = b as isize;
            while ch >= 0 && ch < h as isize && cw >= 0 && cw < w as isize {
                if map[ch as usize][cw as usize] == BLOCK {
                    break;
                }
                if ch as usize != a && cw as usize != b && map[ch as usize][cw as usize] == LAMP {
                    has_lamp[ch as usize][cw as usize][(i + 2) % 4] = true;
                    break;
                }
                if !lighted[ch as usize][cw as usize] {
                    ans += 1;
                }
                lighted[ch as usize][cw as usize] = true;
                ch += dh;
                cw += dw;
            }
        }
    }
    ans.to_string()
}

#[test]
fn test() {
    let cases: Vec<(&str, &str)> = vec![
        (
            r"3 3 2 1
1 1
2 3
2 2",
            "7"
        ),
        (
            r"4 4 3 3
1 2
1 3
3 4
2 3
2 4
3 2",
            "8"
        ),
        (
            r"5 5 5 1
1 1
2 2
3 3
4 4
5 5
4 2",
            "24"
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

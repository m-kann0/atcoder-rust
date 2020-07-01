use std::io::Read;

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
    let map: Vec<Vec<char>> =
        (0..h).map(|_| iterator.next().unwrap().chars().collect()).collect();

    let mut original = map.clone();
    for ch in 0..h {
        for cw in 0..w {
            if is_next_to(h, w, &map, ch, cw, '.') {
                original[ch as usize][cw as usize] = '.';
            }
        }
    }

    let mut shrinked = original.clone();
    for ch in 0..h {
        for cw in 0..w {
            if is_next_to(h, w, &original, ch, cw, '#') {
                shrinked[ch as usize][cw as usize] = '#';
            }
        }
    }

    if map == shrinked {
        let mut result = String::from("possible\n");
        for line in original {
            for c in line {
                result.push(c);
            }
            result.push('\n');
        }
        result.trim().to_string()
    } else {
        "impossible".to_string()
    }
}

fn is_next_to(h: isize, w: isize, map: &Vec<Vec<char>>, ch: isize, cw: isize, c: char) -> bool {
    let dh: Vec<isize> = vec![-1, -1, -1, 0, 0, 1, 1, 1];
    let dw: Vec<isize> = vec![-1, 0, 1, -1, 1, -1, 0, 1];

    for i in 0..8 {
        let nh = ch + dh[i];
        let nw = cw + dw[i];
        if nh < 0 || nh >= h || nw < 0 || nw >= w {
            continue;
        }
        if map[nh as usize][nw as usize] == c {
            return true;
        }
    }

    return false;
}

#[test]
fn test() {
    let cases: Vec<(&str, &str)> = vec![
        (
            r"4 4
##..
##..
..##
..##",
            "possible
#...
....
....
...#"
        ),
        (
            r"4 4
###.
####
..##
..##",
            "possible
##..
....
...#
...#"
        ),
        (
            r"4 4
###.
##.#
..##
..##",
            "impossible"
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
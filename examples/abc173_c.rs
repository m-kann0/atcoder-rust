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
    let k: usize = iterator.next().unwrap().parse().unwrap();
    let mut map: Vec<Vec<char>> = Vec::new();
    for _ in 0..h {
        map.push(iterator.next().unwrap().chars().collect());
    }

    let mut ans: usize = 0;
    for i in 0..2_i32.pow(h as u32) {
        let bi: Vec<char> = binary_string(i as usize, h).chars().collect();
        for j in 0..2_i32.pow(w as u32) {
            let bj: Vec<char> = binary_string(j as usize, w).chars().collect();
            let mut count = 0;
            for hi in 0..h {
                for wi in 0..w {
                    if bi[hi] != '1' && bj[wi] != '1'
                        && map[hi][wi] == '#' {
                        count += 1;
                    }
                }
            }
            if count == k {
                ans += 1;
            }
        }
    }
    ans.to_string()
}

fn binary_string(n: usize, keta: usize) -> String {
    return format!("{:0>1$b}", n, keta);
}

#[test]
fn test() {
    let cases: Vec<(&str, &str)> = vec![
        (
            r"2 3 2
..#
###",
            "5"
        ),
        (
            r"2 3 4
..#
###",
            "1"
        ),
        (
            r"2 2 3
##
##",
            "0"
        ),
        (
            r"6 6 8
..##..
.#..#.
#....#
######
#....#
#....#",
            "208"
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
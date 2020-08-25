use std::io::Read;
use std::collections::HashSet;

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
    let m: usize = iterator.next().unwrap().parse().unwrap();
    let mut bomb: HashSet<(usize, usize)> = HashSet::new();
    let mut row: Vec<usize> = vec![0; h];
    let mut col: Vec<usize> = vec![0; w];
    for _ in 0..m {
        let hi = iterator.next().unwrap().parse::<usize>().unwrap() - 1;
        let wi = iterator.next().unwrap().parse::<usize>().unwrap() - 1;
        row[hi] += 1;
        col[wi] += 1;
        bomb.insert((hi, wi));
    }

    let mut max_row = 0;
    // let mut max_row_index = 0;
    let mut max_row_indexes = Vec::new();
    for i in 0..h {
        if row[i] > max_row {
            max_row = row[i];
            // max_row_index = i;
            max_row_indexes = vec![i];
        } else if row[i] == max_row {
            max_row_indexes.push(i);
        }
    }

    let mut max_col = 0;
    // let mut max_col_index = 0;
    let mut max_col_indexes = Vec::new();
    for i in 0..w {
        if col[i] > max_col {
            max_col = col[i];
            // max_col_index = i;
            max_col_indexes = vec![i];
        } else if col[i] == max_col {
            max_col_indexes.push(i);
        }
        // } else if col[i] == max_col
        //     && bomb.contains(&(max_row_index, max_col_index))
        //     && !bomb.contains(&(max_row_index, i)) {
        //     max_col = col[i];
        //     max_col_index = i;
        // }
    }

    let mut is_happy = false;
    let mut count: usize = 0;
    for &ri in &max_row_indexes {
        for &ci in &max_col_indexes {
            if !bomb.contains(&(ri, ci)) {
                is_happy = true;
                break;
            }
            count += 1;
            if count > 100000000 {
                break;
            }
        }
    }

    let mut ans = max_row + max_col;
    if !is_happy {
        ans -= 1;
    }
    ans.to_string()
}

#[test]
fn test() {
    let cases: Vec<(&str, &str)> = vec![
        (
            r"2 3 3
2 2
1 1
1 3",
            "3"
        ),
        (
            r"3 3 4
3 3
3 1
1 1
1 2",
            "3"
        ),
        (
            r"5 5 10
2 5
4 3
2 3
5 5
2 2
5 4
5 3
5 1
3 5
1 4",
            "6"
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
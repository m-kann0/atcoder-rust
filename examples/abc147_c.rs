use std::io::Read;
use std::cmp::max;

fn main() {
    let mut buf = String::new();
    std::io::stdin().read_to_string(&mut buf).unwrap();

    let answer = solve(&buf);

    println!("{}", answer);
}

fn solve(input: &str) -> String {
    let mut iterator = input.split_whitespace();

    let n: usize = iterator.next().unwrap().parse().unwrap();
    let mut evidences: Vec<Vec<isize>> = Vec::new();
    for _i in 0..n {
        let mut evidence: Vec<isize> = vec![-1; n];

        let a: usize = iterator.next().unwrap().parse().unwrap();
        for _j in 0..a {
            let x: usize = iterator.next().unwrap().parse().unwrap();
            let y: isize = iterator.next().unwrap().parse().unwrap();

            evidence[x - 1] = y;
        }

        evidences.push(evidence)
    }

    let mut ans: usize = 0;

    let m: usize = 2_usize.pow(n as u32);
    for i in 0..m {
        let mut valid = true;

        let vec = binary_vec(i, n);
        for j in 0..n {
            if vec[j] != 1 {
                continue;
            }
            for k in 0..n {
                if evidences[j][k] == -1 {
                    continue;
                }
                if evidences[j][k] != vec[k] {
                    valid = false;
                    break;
                }
            }
        }

        if valid {
            ans = max(ans, count(vec))
        }
    }

    return ans.to_string();
}

fn binary_vec(i: usize, len: usize) -> Vec<isize> {
    let binary_str = format!("{:0>1$b}", i, len);
    let result: Vec<isize> = binary_str.chars().map(|c| if c == '0' { 0 } else { 1 }).collect();
    return result;
}

fn count(vec: Vec<isize>) -> usize {
    let mut result = 0;
    for i in vec.iter() {
        if i == &1 {
            result += 1;
        }
    }
    return result;
}

#[test]
fn test() {
    assert_eq!(
        solve(
            r"3
1
2 1
1
1 1
1
2 0"
        ),
        "2"
    );
    assert_eq!(
        solve(
            r"3
2
2 1
3 0
2
3 1
1 0
2
1 1
2 0"
        ),
        "0"
    );
    assert_eq!(
        solve(
            r"2
1
2 0
1
1 0"
        ),
        "1"
    );
}

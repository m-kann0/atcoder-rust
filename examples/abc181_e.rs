use std::io::Read;
use std::cmp::min;

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
    let mut h: Vec<isize> = (0..n).map(|_| iterator.next().unwrap().parse().unwrap()).collect();
    let mut w: Vec<isize> = (0..m).map(|_| iterator.next().unwrap().parse().unwrap()).collect();

    h.sort();
    w.sort();

    let mut sum1: Vec<isize> = vec![0; (n + 1) / 2];
    for i in 0..(n / 2) {
        sum1[i + 1] = h[2 * i + 1] - h[2 * i] + sum1[i];
    }
    // eprintln!("sum1 = {:?}", sum1);

    h.reverse();

    let mut sum2: Vec<isize> = vec![0; (n + 1) / 2];
    for i in 0..(n / 2) {
        sum2[i + 1] = h[2 * i] - h[2 * i + 1] + sum2[i];
    }
    sum2.reverse();
    // eprintln!("sum2 = {:?}", sum2);

    h.reverse();

    let mut ans: isize = std::isize::MAX;
    for i in 0..m {
        let mut ok: isize = 0;
        let mut ng: isize = n as isize;
        while (ok - ng).abs() > 1 {
            let mid = (ok + ng) / 2;
            if h[mid as usize] <= w[i] {
                ok = mid;
            } else {
                ng = mid;
            }
        }
        // eprintln!("w[i] = {:?}", w[i]);
        // eprintln!("ok = {:?}", ok);
        let j = ok as usize;
        let mut a = sum1[(j + 1) / 2] + sum2[(j + 1) / 2];
        a += if j % 2 == 0 {
            (w[i] - h[j]).abs()
        } else {
            (w[i] - h[j + 1]).abs()
        };
        // eprintln!("a = {:?}", a);
        ans = min(ans, a);
    }
    ans.to_string()
}

#[test]
fn test() {
    let cases: Vec<(&str, &str)> = vec![
//         (
//             r"5 3
// 1 2 3 4 7
// 1 3 8",
//             "3"
//         ),
//         (
//             r"5 3
// 1 2 3 4 7
// 1 3 8",
//             "3"
//         ),
        (
            r"7 7
31 60 84 23 16 13 32
96 80 73 76 87 57 29",
            "34"
        ),
//         (
//             r"15 10
// 554 525 541 814 661 279 668 360 382 175 833 783 688 793 736
// 496 732 455 306 189 207 976 73 567 759",
//             "239"
//         ),
//         (
//             r"5 3
// 1 2 3 7 7
// 1 3 8",
//             "1"
//         ),
//         (
//             r"1 3
// 2
// 1 3 8",
//             "1"
//         ),
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
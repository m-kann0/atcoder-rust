#![allow(non_snake_case)]

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
    let p: Vec<usize> = (0..n).map(|_| iterator.next().unwrap().parse().unwrap()).collect();

    {
        let mut a = p.clone();
        let c = one::merge_sort(&mut a, 0, n);
        if c != n - 1 {
            return "-1".to_string();
        }
    }

    let mut result = String::new();
    let mut a = p.clone();
    let c = merge_sort(&mut a, 0, n, &mut result);
    if c != n - 1 {
        return "-1".to_string();
    }
    result
}

mod one {
    pub fn merge_sort(a: &mut Vec<usize>, left: usize, right: usize) -> usize {
        if left + 1 < right {
            let mid = (left + right) / 2;
            let mut count = 0;
            count += merge_sort(a, left, mid);
            count += merge_sort(a, mid, right);
            count += merge(a, left, mid, right);
            count
        } else {
            0
        }
    }

    fn merge(a: &mut Vec<usize>, left: usize, mid: usize, right: usize) -> usize {
        let n1 = mid - left;
        let n2 = right - mid;
        let mut l: Vec<usize> = Vec::new();
        let mut r: Vec<usize> = Vec::new();
        for i in 0..n1 {
            l.push(a[left + i]);
        }
        l.push(std::usize::MAX);
        for i in 0..n2 {
            r.push(a[mid + i]);
        }
        r.push(std::usize::MAX);

        let mut count = 0;

        let mut i = 0;
        let mut j = 0;
        for k in left..right {
            if l[i] <= r[j] {
                a[k] = l[i];
                i += 1;
            } else {
                a[k] = r[j];
                j += 1;
                count += n1 - i;
            }
        }

        count
    }
}

fn merge_sort(a: &mut Vec<usize>, left: usize, right: usize, result: &mut String) -> usize {
    if left + 1 < right {
        let mid = (left + right) / 2;
        let mut count = 0;
        count += merge_sort(a, left, mid, result);
        count += merge_sort(a, mid, right, result);
        count += merge(a, left, mid, right, result);
        count
    } else {
        0
    }
}

fn merge(a: &mut Vec<usize>, left: usize, mid: usize, right: usize, result: &mut String) -> usize {
    let n1 = mid - left;
    let n2 = right - mid;
    let mut l: Vec<usize> = Vec::new();
    let mut r: Vec<usize> = Vec::new();
    for i in 0..n1 {
        l.push(a[left + i]);
    }
    l.push(std::usize::MAX);
    for i in 0..n2 {
        r.push(a[mid + i]);
    }
    r.push(std::usize::MAX);

    let mut count = 0;

    let mut i = 0;
    let mut j = 0;
    for k in left..right {
        if l[i] <= r[j] {
            a[k] = l[i];
            i += 1;
        } else {
            a[k] = r[j];

            // eprintln!("k = {:?}", k);
            // eprintln!("mid + j = {:?}", mid + j);
            // eprintln!("n1 - i = {:?}", n1 - i);

            let mut x = mid + j;
            for _ in 0..(n1 - i) {
                result.push_str(&format!("{}\n", x));
                x -= 1;
            }

            j += 1;
            count += n1 - i;
        }
    }

    count
}


#[test]
fn test() {
    let cases: Vec<(&str, &str)> = vec![
        (
            r"5
2 4 1 5 3",
            "4
2
3
1"
        ),
        (
            r"5
5 4 3 2 1",
            "-1"
        ),
    ];

    let mut all_ok = true;
    for (i, case) in cases.iter().enumerate() {
        print!("case {} : ", i);

        let expected = case.1;
        let actual = solve(case.0);

        if expected.trim() == actual.trim() {
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
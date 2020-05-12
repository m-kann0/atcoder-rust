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
    let mut k: usize = iterator.next().unwrap().parse().unwrap();

    let a: Vec<usize> = (0..n).map(|_| iterator.next().unwrap().parse().unwrap()).collect();

    let mut sum: Vec<usize> = vec![0; n];
    sum[0] = a[0];
    for i in 1..n {
        sum[i] = sum[i - 1] + a[i];
    }

    let mut ans: usize = 0;
    for i in 0..n {
        let result = binary_search(&sum, |x| *x >= k);
        if let Result::Ok(index) = result {
            ans += n - index;
        }
        k += a[i]
    }

    return ans.to_string();
}

fn binary_search<E, P>(vec: &Vec<E>, predicate: P) -> Result<usize, ()>
    where
        E: PartialOrd,
        P: Fn(&E) -> bool,
{
    let mut ng: isize = -1;
    let mut ok: isize = vec.len() as isize;
    while (ok - ng).abs() > 1 {
        let mid = (ok + ng) / 2;
        if predicate(&vec[mid as usize]) {
            ok = mid;
        } else {
            ng = mid;
        }
    }
    return if ok >= 0 && ok < vec.len() as isize {
        Ok(ok as usize)
    } else {
        Err(())
    };
}

#[test]
fn test() {
    let cases: Vec<(&str, &str)> = vec![
        (
            r"4 10
6 1 2 7",
            "2"
        ),
        (
            r"3 5
3 3 3",
            "3"
        ),
        (
            r"10 53462
103 35322 232 342 21099 90000 18843 9010 35221 19352",
            "36"
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
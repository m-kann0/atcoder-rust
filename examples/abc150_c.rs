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
    let mut p: Vec<usize> = Vec::with_capacity(n);
    let mut q: Vec<usize> = Vec::with_capacity(n);

    for _ in 0..n {
        p.push(iterator.next().unwrap().parse().unwrap());
    }
    for _ in 0..n {
        q.push(iterator.next().unwrap().parse().unwrap());
    }

    let perms: Vec<Vec<usize>> = make(n);

    let a: usize = perms.binary_search(&p).unwrap();
    let b: usize = perms.binary_search(&q).unwrap();

    return if a > b {
        format!("{}", a - b)
    } else {
        format!("{}", b - a)
    }
}

fn make(n: usize) -> Vec<Vec<usize>> {
    let mut original = Vec::new();
    for i in 1..(n + 1) {
        original.push(i)
    }

    let mut result = Vec::new();
    _make(original, Vec::new(), &mut result);
    return result;
}

fn _make(candidate: Vec<usize>, perm: Vec<usize>, result: &mut Vec<Vec<usize>>) {
    if candidate.is_empty() {
        result.push(perm);
        return;
    }

    for i in 0..candidate.len() {
        let mut c = candidate.clone();
        let mut p = perm.clone();
        p.push(c.remove(i));
        _make(c, p, result);
    }
}

#[test]
fn test() {
    assert_eq!(
        solve(
            r"3
1 3 2
3 1 2"
        ),
        "3"
    );
    assert_eq!(
        solve(
            r"8
7 3 5 4 2 1 6 8
3 8 2 5 4 6 7 1"
        ),
        "17517"
    );
    assert_eq!(
        solve(
            r"3
1 2 3
1 2 3"
        ),
        "0"
    );
}

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

    let mut vec: Vec<(i64, i64)> = Vec::new();
    for _i in 0..n {
        let x: i64 = iterator.next().unwrap().parse().unwrap();
        let y: i64 = iterator.next().unwrap().parse().unwrap();
        vec.push((x, y))
    }

    let perm = make(n);
    let mut total: f64 = 0.0;
    for p in &perm {
        let mut sub_total: f64 = 0.0;

        let mut current = vec[p[0] - 1];
        for i in 1..n {
            let next = vec[p[i] - 1];
            sub_total += (((current.0 - next.0).pow(2) + (current.1 - next.1).pow(2)) as f64).sqrt();
            current = next;
        }

        total += sub_total;
    }

    let answer: f64 = total / (perm.len() as f64);

    return format!("{:.10}", answer);
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
0 0
1 0
0 1"
        ),
        "2.2761423749"
    );
    assert_eq!(
        solve(
            r"2
-879 981
-866 890"
        ),
        "91.9238815543"
    );
    assert_eq!(
        solve(
            r"8
-406 10
512 859
494 362
-955 -475
128 553
-986 -885
763 77
449 310"
        ),
        "7641.9817824387"
    );
}

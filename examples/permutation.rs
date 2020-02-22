// 順列を生成する
fn main() {
    let result = make(4);
    println!("{:?}", result);

    let p: Vec<usize> = vec![1, 4, 2, 3];
    let p_index = result.binary_search(&p).unwrap();
    println!("{}", p_index);
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

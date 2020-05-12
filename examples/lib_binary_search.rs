/// 2分探索する。
///
/// predicateがtrueを返す、最小のindexを返却する。
/// 全ての要素が条件を満たさない場合、Result::Err(())を返却する。
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
    let vec: Vec<usize> = vec![1, 3, 4, 8, 11, 15];
    assert_eq!(binary_search(&vec, |x| *x >= 8), Result::Ok(3));
    assert_eq!(binary_search(&vec, |x| *x >= 1), Result::Ok(0));
    assert_eq!(binary_search(&vec, |x| *x >= 20), Result::Err(()));
}

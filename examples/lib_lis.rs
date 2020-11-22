/// 最長増加部分列（LIS：Longest Increasing Subsequence）の長さを求める。
/// 蟻本p65
fn main() {
    let a = vec![4, 2, 3, 1, 5];
    println!("{}", lis::lis(&a, std::usize::MAX));
}

mod lis {
    pub fn lis<T: PartialOrd + PartialEq + Copy + Clone>(v: &Vec<T>, inf: T) -> usize {
        let mut dp = vec![inf; v.len()];
        for &x in v {
            let mut ok: isize = v.len() as isize - 1;
            let mut ng: isize = -1;
            while (ok - ng).abs() > 1 {
                let mid = (ok + ng) / 2;
                if x <= dp[mid as usize] {
                    ok = mid;
                } else {
                    ng = mid;
                }
            }
            dp[ok as usize] = x.clone();
        }

        let mut ans = 0;
        for i in 0..v.len() {
            if dp[i] < inf {
                ans = i + 1;
            }
        }
        ans
    }
}

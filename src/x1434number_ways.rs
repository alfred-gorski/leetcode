impl Solution {
    pub fn number_ways(hats: Vec<Vec<i32>>) -> i32 {
        let n = hats.len();
        let mut dp: Vec<i64> = Vec::with_capacity(1 << n);
        dp.resize_with(dp.capacity(), || 0);
        dp[0] = 1;
        let mut s = Vec::with_capacity(41);
        s.resize_with(s.capacity(), || Vec::new());
        for i in 0..n {
            for &hat in hats[i].iter() {
                s[hat as usize].push(i);
            }
        }

        for v in s.iter_mut() {
            v.sort();
        }
        // 给帽子找人, v: 每个帽子对应的需求
        for v in s.iter() {
            // v 是第 i 个帽子
            for state in (0..(1 << n)).rev() {
                // state 第几位为1就是第几位带了帽子
                // 对于每一顶帽子尝试分配给一个当前
                // 还没有帽子且对这个帽子有需求的人
                for &k in v {
                    if state & (1 << k) == 0 {
                        let nxt = state + (1 << k);
                        dp[nxt] += dp[state];
                        dp[nxt] %= 1e9 as i64 + 7;
                    }
                }
            }
        }
        return dp[(1 << n) - 1] as i32;
    }
}

pub struct Solution;

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn test() {
        assert_eq!(
            Solution::number_ways(vec![vec![3, 4], vec![4, 5], vec![5]]),
            1
        );
        assert_eq!(Solution::number_ways(vec![vec![3, 5, 1], vec![3, 5]]), 4);
    }
}

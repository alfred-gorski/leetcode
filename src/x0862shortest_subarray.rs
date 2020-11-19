use std::{cmp::min, collections::VecDeque};

pub struct Solution;

impl Solution {
    pub fn shortest_subarray(a: Vec<i32>, k: i32) -> i32 {
        let n = a.len();
        let mut p: Vec<i64> = vec![0; n + 1];
        for (i, &x) in a.iter().enumerate() {
            p[i + 1] = p[i] + x as i64;
        }
        let mut ans = n as i64 + 1;
        let mut monoq = VecDeque::with_capacity(n/2);
        for i in 0..n + 1 {
            while !monoq.is_empty() && p[i] <= p[*monoq.back().unwrap()] {
                monoq.pop_back();
            }
            while !monoq.is_empty() && p[i] >= (p[*monoq.front().unwrap()] + k as i64) {
                ans = min(ans, (i - monoq.pop_front().unwrap()) as i64);
            }
            monoq.push_back(i);
        }
        if ans < n as i64 + 1 {
            return ans as i32;
        } else {
            return -1;
        }
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn test() {
        assert_eq!(Solution::shortest_subarray(vec![1], 1), 1);
        assert_eq!(Solution::shortest_subarray(vec![1, 2], 4), -1);
        assert_eq!(Solution::shortest_subarray(vec![2, -1, 2], 3), 3);
    }
}

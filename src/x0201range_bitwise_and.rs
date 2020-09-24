pub struct Solution;

impl Solution {
    pub fn range_bitwise_and(m: i32, n: i32) -> i32 {
        let mut cnt = 0;
        let (mut m, mut n) = (m, n);
        while m != n {
            m = m >> 1;
            n = n >> 1;
            cnt += 1;
        }
        m << cnt
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn test() {
        assert_eq!(Solution::range_bitwise_and(5, 7), 4);
        assert_eq!(Solution::range_bitwise_and(0, 1), 0);
    }
}

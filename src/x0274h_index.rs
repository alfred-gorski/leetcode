pub struct Solution;

impl Solution {
    pub fn h_index(mut citations: Vec<i32>) -> i32 {
        let mut h = 0;
        citations.sort_unstable_by(|a, b| b.cmp(a));
        while h < citations.len() && h < citations[h] as usize {
            h = h + 1;
        }
        return h as i32;
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn test() {
        assert_eq!(Solution::h_index(vec![3, 0, 6, 1, 5]), 3);
        assert_eq!(Solution::h_index(vec![1, 3, 1]), 1);
    }
}

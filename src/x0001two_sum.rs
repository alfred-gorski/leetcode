use std::collections::BTreeMap;

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut map = BTreeMap::new();
        let mut ret = Vec::new();
        for (i, n) in nums.iter().enumerate() {
            if let Some(&idx) = map.get(&(target - n)) {
                ret.extend(vec![idx as i32, i as i32]);
                break;
            } else {
                map.insert(n, i);
            }
        }
        ret
    }
}

pub struct Solution;

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn test() {
        assert_eq!(Solution::two_sum(vec![2, 7, 11, 5], 9), vec![0, 1]);
    }
}

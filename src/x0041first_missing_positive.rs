impl Solution {
    pub fn first_missing_positive(nums: Vec<i32>) -> i32 {
        let len = nums.len() as i32;
        if len == 0 {
            return 1;
        }
        let mut nums = nums
            .iter()
            .map(|v| {
                if *v > len as i32 || *v <= 0 {
                    len + 1
                } else {
                    *v
                }
            })
            .collect::<Vec<_>>();
        for i in 0..len {
            let v = nums[i as usize].abs() as usize;
            if v <= len as usize {
                nums[v - 1] = -nums[v - 1].abs();
            }
        }
        for (i, v) in nums.iter().enumerate() {
            if *v > 0 {
                return (i + 1) as i32;
            }
        }
        len + 1
    }
}

pub struct Solution;

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn test() {
        assert_eq!(Solution::first_missing_positive(vec![3, 4, -1, 1]), 2);
        assert_eq!(Solution::first_missing_positive(vec![1, 2, 0]), 3);
        assert_eq!(Solution::first_missing_positive(vec![7, 8, 9, 11, 12]), 1);
        assert_eq!(Solution::first_missing_positive(vec![]), 1);
    }
}

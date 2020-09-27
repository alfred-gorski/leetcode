pub struct Solution;

impl Solution {
    pub fn path_in_zig_zag_tree(label: i32) -> Vec<i32> {
        let mut ans = Vec::new();
        let mut label = label;
        while label > 0 {
            ans.push(label);
            label /= 2;
        }

        ans.reverse();
        if ans.len() % 2 == 0 {
            for (i, n) in ans.iter_mut().enumerate() {
                if i % 2 == 0 {
                    *n = 3 * (1 << i) - 1 - *n;
                }
            }
        }else{
            for (i, n) in ans.iter_mut().enumerate() {
                if i % 2 == 1 {
                    *n = 3 * (1 << i) - 1 - *n;
                }
            }
        }

        ans
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn test() {
        assert_eq!(Solution::path_in_zig_zag_tree(14), vec![1, 3, 4, 14]);
        assert_eq!(Solution::path_in_zig_zag_tree(26), vec![1, 2, 6, 10, 26]);
    }
}

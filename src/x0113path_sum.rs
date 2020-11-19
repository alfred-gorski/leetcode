use leetcode_prelude::TreeNode;
use std::{cell::RefCell, rc::Rc};

pub struct Solution;

impl Solution {
    pub fn path_sum(root: Option<Rc<RefCell<TreeNode>>>, sum: i32) -> Vec<Vec<i32>> {
        let mut path: Vec<i32> = Vec::new();
        let mut ans = Vec::new();

        Self::dfs(root.as_ref(), sum, &mut path, &mut ans);

        ans
    }

    fn dfs(
        root: Option<&Rc<RefCell<TreeNode>>>,
        mut sum: i32,
        path: &mut Vec<i32>,
        ans: &mut Vec<Vec<i32>>,
    ) {
        if let Some(root) = root {
            let root = root.borrow();
            let val = root.val;
            path.push(val);
            sum -= val;
            if root.left.is_none() && root.right.is_none() && sum == 0 {
                ans.push(path.to_vec());
            }
            Self::dfs(root.left.as_ref(), sum, path, ans);
            Self::dfs(root.right.as_ref(), sum, path, ans);
            path.pop();
        } else {
            return;
        }
    }
}

#[cfg(test)]
mod test {
    use super::Solution;
    use leetcode_prelude::{btree, TreeNode};
    use std::{cell::RefCell, rc::Rc};

    #[test]
    fn test() {
        let tree: Option<Rc<RefCell<TreeNode>>> =
            btree![5, 4, 8, 11, null, 13, 4, 7, 2, null, null, 5, 1];

        assert_eq!(
            Solution::path_sum(tree, 22),
            vec![vec![5, 4, 11, 2], vec![5, 8, 4, 5]]
        );
    }
}

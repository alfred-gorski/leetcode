pub struct Solution;

impl Solution {
    pub fn find_mode(root: Vec<i32>) -> Vec<i32> {
        let mut res = Vec::new();
        let (mut curcnt, mut maxcnt) = (1, 1);
        let mut pre = *root.first().unwrap();
        res.push(pre);
        for &cur in root[1..].iter() {
            if cur == pre {
                curcnt += 1;
            } else {
                pre = cur;
                curcnt = 1;
            }
            if curcnt > maxcnt {
                maxcnt = curcnt;
                res.clear();
                res.push(cur);
            } else if curcnt == maxcnt {
                res.push(cur);
            }
        }
        res
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn test() {
        assert_eq!(Solution::find_mode(vec![1, 2, 2]), vec![2]);
        assert_eq!(Solution::find_mode(vec![1, 1, 2, 2]), vec![1, 2]);
        assert_eq!(Solution::find_mode(vec![1, 2, 2, 3]), vec![2]);
        assert_eq!(Solution::find_mode(vec![1, 2, 2, 3, 3]), vec![2, 3]);
        assert_eq!(Solution::find_mode(vec![1, 2, 2, 3, 3, 5]), vec![2, 3]);
        assert_eq!(
            Solution::find_mode(vec![1, 2, 2, 3, 3, 5, 5]),
            vec![2, 3, 5]
        );
    }
}

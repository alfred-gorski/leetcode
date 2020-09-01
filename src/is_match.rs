pub struct Solution;

impl Solution {
    pub fn is_match(s: String, p: String) -> bool {
        let s = Vec::from(s);
        let p = Vec::from(p);
        let m = s.len();
		let n = p.len();
		
        let mut f = vec![vec![false; n+1]; m+1];
        f[0][0] = true;

        let matches = |i, j| {
            if i == 0 {
                false
            } else if p[j - 1] == '.' as u8 {
                true
            } else {
                s[i - 1] == p[j - 1]
            }
        };

        for i in 0..=m {
            for j in 1..=n {
                if p[j - 1] == '*' as u8 {
                    f[i][j] |= f[i][j - 2];
                    if matches(i, j - 1) {
                        f[i][j] |= f[i - 1][j];
                    }
                } else {
                    if matches(i, j) {
                        f[i][j] |= f[i - 1][j - 1];
                    }
                }
            }
        }
        f[m][n]
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn test() {
        assert_eq!(Solution::is_match("aa".to_string(), "a".to_string()), false);
        assert_eq!(Solution::is_match("aa".to_string(), "a*".to_string()), true);
        assert_eq!(
            Solution::is_match("aab".to_string(), "c*a*b".to_string()),
            true
        );
        assert_eq!(Solution::is_match("ab".to_string(), ".*".to_string()), true);
        assert_eq!(
            Solution::is_match("mississippi".to_string(), "mis*is*p*.".to_string()),
            false
        );
    }
}

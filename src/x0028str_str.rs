use std::usize;

pub struct Solution;

impl Solution {
    pub fn str_str(haystack: String, needle: String) -> i32 {
        let mut h: usize = 0;
        let mut n: usize = 0;

        let haystack = haystack.as_bytes().to_vec();

		let needle = needle.as_bytes().to_vec();
		if needle.len() == 0 {
            return 0;
        }
        let next = Self::get_next(&needle);

        while h < haystack.len() {
            if haystack[h] == needle[n] {
                h += 1;
                n += 1;
            } else if n > 0 {
                n = next[n - 1];
            } else {
                h += 1;
            }

            if n == needle.len() {
                return h as i32 - n as i32;
            }
        }
        return -1;
    }

    fn get_next(p: &Vec<u8>) -> Vec<usize> {
        let mut next = Vec::with_capacity(p.len());
        next.push(0);
        let mut now: usize = 0;
        let mut x: usize = 1;

        while x < p.len() {
            if p[now] == p[x] {
                now += 1;
                x += 1;
                next.push(now);
            } else if now > 0 {
                now = next[now - 1]
            } else {
                next.push(0);
                x += 1;
            }
        }

        next
    }
}

#[cfg(test)]
mod test {

    #[test]
    fn test() {
        assert_eq!(
            super::Solution::get_next(&"abcabdddabcabc".as_bytes().to_vec()),
            vec![0, 0, 0, 1, 2, 0, 0, 0, 1, 2, 3, 4, 5, 3]
        );
        assert_eq!(
            super::Solution::str_str("hello".to_string(), "ll".to_string()),
            2
        );
        assert_eq!(
            super::Solution::str_str("aaaaa".to_string(), "bba".to_string()),
            -1
		);
		assert_eq!(
            super::Solution::str_str("".to_string(), "".to_string()),
            0
        );
    }
}

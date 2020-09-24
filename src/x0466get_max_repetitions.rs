use std::collections::HashMap;
impl Solution {
    pub fn get_max_repetitions(s1: String, n1: i32, s2: String, n2: i32) -> i32 {
        if n1 == 0 {
            return 0;
        }
        let (s1, s2) = (Vec::from(s1), Vec::from(s2));
        let mut s1cnt: usize = 0;
        let mut index: usize = 0;
        let mut s2cnt: usize = 0;
        let mut ans;
        let rest;
        let mut recall: HashMap<usize, (usize, usize)> = HashMap::new();
        let pre_loop;
        let in_loop;
        loop {
            s1cnt += 1;
            for ch in s1.iter() {
                if ch == &s2[index] {
                    index += 1;
                }
                if index == s2.len() {
                    s2cnt += 1;
                    index = 0;
                }
            }
            if s1cnt as i32 == n1 {
                return (s2cnt as i32) / n2;
            }
            if let Some(res) = recall.get(&index) {
                pre_loop = res.clone();
                in_loop = (s1cnt - pre_loop.0, s2cnt - pre_loop.1);
                break;
            } else {
                recall.insert(index, (s1cnt, s2cnt));
            }
        }
        ans = pre_loop.1 + (n1 as usize - pre_loop.0) / in_loop.0 * in_loop.1;
        rest = (n1 as usize - pre_loop.0) % in_loop.0;

        for _i in 0..rest {
            for ch in s1.iter() {
                if ch == &s2[index] {
                    index += 1;
                    if index == s2.len() {
                        ans += 1;
                        index = 0;
                    }
                }
            }
        }

        return (ans as i32) / n2;
    }
}

pub struct Solution;

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn test() {
        assert_eq!(
            Solution::get_max_repetitions(String::from("acb"), 4, String::from("ab"), 2),
            2
        );
    }
}

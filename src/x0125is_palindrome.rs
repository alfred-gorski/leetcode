pub struct Solution;

impl Solution {
    pub fn is_palindrome(s: String) -> bool {
        let (mut itr, mut ritr) = (s.chars(), s.chars().rev());
        let check_next = |x: &mut std::iter::Rev<core::str::Chars>| loop {
            match x.next() {
                Some(x) => {
                    if x.is_ascii_digit() {
                        return Some(x);
                    } else if x.is_ascii_alphabetic() {
                        return Some(x.to_ascii_lowercase());
                    }
                }
                None => {
                    return None;
                }
            }
        };

        loop {
            match itr.next() {
                Some(x) => {
                    if x.is_ascii_alphanumeric() {
                        match check_next(&mut ritr) {
                            Some(y) => {
                                if x.to_ascii_lowercase() != y {
                                    return false;
                                }
                            }
                            None => {
                                return false;
                            }
                        }
                    }
                }
                None => {
                    return true;
                }
            };
        }
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn test() {
        assert_eq!(
            Solution::is_palindrome(String::from("A man, a plan, a canal: Panama")),
            true
        );
        assert_eq!(Solution::is_palindrome(String::from("race a car")), false);
    }
}

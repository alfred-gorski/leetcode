pub struct Solution;

impl Solution {
    pub fn matrix_score(a: Vec<Vec<i32>>) -> i32 {
        let mut a = a;
        for r in a.iter_mut() {
            if *r.first().unwrap() == 0 {
                for c in r.iter_mut() {
                    if *c == 0 {
                        *c = 1;
                    } else {
                        *c = 0;
                    }
                }
            }
        }

        let mut sum = vec![0; a[0].len()];

        for r in a.iter() {
            for (ic, c) in r.iter().enumerate() {
                sum[ic] += c;
            }
        }

        for (c, csum) in sum.iter().enumerate() {
            if (*csum as usize) < a.len() / 2 + 1 {
                for r in a.iter_mut() {
                    if r[c] == 0 {
                        r[c] = 1;
                    } else {
                        r[c] = 0;
                    }
                }
            }
        }
        let mut res = 0;
        let mut rres = 0;
        for r in a.iter() {
            for c in r.iter() {
                rres += c;
                rres = rres << 1;
            }
            res += rres>>1;
            rres = 0;
        }
        res
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn test() {
        assert_eq!(
            Solution::matrix_score(vec![vec![0, 0, 1, 1], vec![1, 0, 1, 0], vec![1, 1, 0, 0]]),
            39
        );
    }
}

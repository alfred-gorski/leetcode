pub struct Solution;
impl Solution {
    pub fn corp_flight_bookings(bookings: Vec<Vec<i32>>, n: i32) -> Vec<i32> {
        let n = n as usize;
        let mut v = vec![0; n + 1];
        let mut sum = 0;

        for i in bookings.iter() {
            v[i[0] as usize - 1] += i[2];
            v[i[1] as usize] -= i[2];
        }

        for i in 0..n {
            sum += v[i];
            v[i] = sum;
        }
        v.pop();
        v
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn test() {
        assert_eq!(
            Solution::corp_flight_bookings(vec![vec![1, 2, 10], vec![2, 3, 20], vec![2, 5, 25]], 5),
            vec![10, 55, 45, 25, 25]
        );
    }
}

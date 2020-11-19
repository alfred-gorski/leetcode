use std::{
    cmp::Ordering,
    collections::{BinaryHeap, HashMap},
};

pub struct Solution;

impl Solution {
    pub fn find_cheapest_price(n: i32, flights: Vec<Vec<i32>>, src: i32, dst: i32, k: i32) -> i32 {
        let mut flight = Vec::with_capacity(flights.len());
        let mut descprice = vec![vec![i32::MAX; k as usize + 3]; n as usize];
        let mut flight_map: HashMap<i32, Vec<(i32, i32)>> = HashMap::new();
        let mut heap = BinaryHeap::new();

        for f in flights {
            flight.push(Edge {
                src: f[0],
                dst: f[1],
                w: f[2],
            });
        }

        for f in flight.iter_mut() {
            // flight_map.insert(f.src, (f.dst, f.w));
            flight_map
                .entry(f.src)
                .and_modify(|v| v.push((f.dst, f.w)))
                .or_insert(vec![(f.dst, f.w)]);
        }

        heap.push(EdgeTuple(src, 0, 0));

        while let Some(EdgeTuple(no, price, trans)) = heap.pop() {
            if trans > k + 1 || price > descprice[no as usize][trans as usize] {
                continue;
            }
            if no == dst {
                return price;
            }

            descprice[no as usize][trans as usize] =
                std::cmp::min(price, descprice[no as usize][trans as usize]);

            if let Some(f) = flight_map.get(&no) {
                for &(destination, weight) in f {
                    heap.push(EdgeTuple(destination, weight + price, trans + 1));
                }
            }
        }
        return -1;
    }
}

struct Edge {
    src: i32,
    dst: i32,
    w: i32,
}

#[derive(Eq, Debug)]
struct EdgeTuple(i32, i32, i32);

impl Ord for EdgeTuple {
    fn cmp(&self, other: &Self) -> Ordering {
        other.1.cmp(&self.1)
    }
}

impl PartialOrd for EdgeTuple {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(other.1.cmp(&self.1))
    }
}

impl PartialEq for EdgeTuple {
    fn eq(&self, other: &Self) -> bool {
        self.1 == other.1
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn test() {
        assert_eq!(
            Solution::find_cheapest_price(
                3,
                vec![vec![0, 1, 100], vec![1, 2, 100], vec![0, 2, 500]],
                0,
                2,
                1
            ),
            200
        );
        assert_eq!(
            Solution::find_cheapest_price(
                3,
                vec![vec![0, 1, 100], vec![1, 2, 100], vec![0, 2, 500]],
                0,
                2,
                0
            ),
            500
        );
        assert_eq!(
            Solution::find_cheapest_price(
                5,
                vec![
                    vec![4, 1, 1],
                    vec![1, 2, 3],
                    vec![0, 3, 2],
                    vec![0, 4, 10],
                    vec![3, 1, 1],
                    vec![1, 4, 3]
                ],
                2,
                1,
                1
            ),
            -1
        );
        assert_eq!(
            Solution::find_cheapest_price(
                5,
                vec![
                    vec![1, 2, 10],
                    vec![2, 0, 7],
                    vec![1, 3, 8],
                    vec![4, 0, 10],
                    vec![3, 4, 2],
                    vec![4, 2, 10],
                    vec![0, 3, 3],
                    vec![3, 1, 6],
                    vec![2, 4, 5]
                ],
                0,
                4,
                1,
            ),
            5
        );
    }
}

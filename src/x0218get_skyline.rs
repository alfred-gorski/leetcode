use std::collections::BTreeMap;

pub struct Solution;

#[derive(PartialEq, PartialOrd, Eq, Ord)]
struct Point {
    x: i32,
    y: i32,
}

impl Solution {
    pub fn get_skyline(buildings: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut all = Vec::with_capacity(2 * buildings.len());
        let mut res = Vec::new();

        for rec in buildings.iter() {
            all.push(Point {
                x: rec[0],
                y: -rec[2],
            });
            all.push(Point {
                x: rec[1],
                y: rec[2],
            });
        }

        all.sort();

        let mut heights = BTreeMap::new();
        heights.insert(0, 1);

        let mut last = (0, 0);

        for p in all {
            if p.y < 0 {
                heights.entry(-p.y).and_modify(|t| *t += 1).or_insert(1);
            } else {
                *heights.get_mut(&p.y).unwrap() -= 1;
                if heights[&p.y] == 0 {
                    heights.remove(&p.y);
                }
            }

            // let max_height = *heights.last().unwrap();
            let max_height = *heights.keys().rev().next().unwrap();

            if last.1 != max_height {
                last = (p.x, max_height);
                res.push(vec![last.0, last.1]);
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
        assert_eq!(
            Solution::get_skyline(vec![
                vec![2, 9, 10],
                vec![3, 7, 15],
                vec![5, 12, 12],
                vec![15, 20, 10],
                vec![19, 24, 8]
            ]),
            vec![
                vec![2, 10],
                vec![3, 15],
                vec![7, 12],
                vec![12, 0],
                vec![15, 10],
                vec![20, 8],
                vec![24, 0]
            ]
        );
        assert_eq!(
            Solution::get_skyline(vec![vec![0, 2, 3], vec![2, 5, 3]]),
            vec![vec![0, 3], vec![5, 0]]
        );
    }
}

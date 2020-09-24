/*
利用并查集构建查找根结点的图
树的每一个非根节点只有一个父节点
附加的边的情况有
1. 指向根结点->有环路

2. 指向非根节点->有一个节点有两个父节点
    2.1 有环路
    2.2 无环路

有环路 -> 新插入的边的两个节点的根结点一致
*/

struct UnionFind {
    fa: Vec<usize>,
}

impl UnionFind {
    fn new(n: usize) -> Self {
        let mut fa = Vec::with_capacity(n);
        for index in 0..n {
            fa.push(index)
        }
        Self { fa }
    }
    fn find(&mut self, index: usize) -> usize {
        if index == self.fa[index] {
            index
        } else {
            self.fa[index] = self.find(self.fa[index]);
            self.fa[index]
        }
    }

    fn merge(&mut self, u: usize, v: usize) {
        let index = self.find(u);
        self.fa[index] = self.find(v);
    }
}

impl Solution {
    pub fn find_redundant_directed_connection(edges: Vec<Vec<i32>>) -> Vec<i32> {
        let mut uf = UnionFind::new(edges.len() + 1);
        let mut fa = Vec::with_capacity(edges.len() + 1);
        for i in 0..=edges.len() {
            fa.push(i)
        }

        let mut conflict = -1;
        let mut cycle = -1;

        for (i, edge) in edges.iter().enumerate() {
            let node1 = edge[0] as usize;
            let node2 = edge[1] as usize;

            if fa[node2] != node2 {
                conflict = i as i32
            } else {
                fa[node2] = node1;
                if uf.find(node1) == uf.find(node2) {
                    cycle = i as i32;
                } else {
                    uf.merge(node1, node2);
                }
            }
        }

        if conflict < 0 {
            let redundant = edges[cycle as usize].clone();
            redundant
        } else {
            let conflict_edge = edges[conflict as usize].clone();
            if cycle >= 0 {
                let redundant = vec![fa[conflict_edge[1] as usize] as i32, conflict_edge[1]];
                redundant
            } else {
                conflict_edge
            }
        }
    }
}

pub struct Solution;

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn test() {
        assert_eq!(
            Solution::find_redundant_directed_connection(vec![vec![1, 2], vec![1, 3], vec![2, 3]]),
            vec![2, 3]
        );
    }
}

use leetcode_prelude::ListNode;
use std::{cmp::Reverse, collections::BinaryHeap};

pub struct Solution;

impl Solution {
    //
    pub fn merge_k_lists(lists: Vec<Option<Box<ListNode>>>) -> Option<Box<ListNode>> {
        let mut q = BinaryHeap::new();

        for list in lists {
            if let Some(node) = list {
                q.push(Reverse(node));
            }
        }

        let mut head = Box::new(ListNode::new(0));
        let mut p = &mut head;
        while let Some(Reverse(mut node)) = q.pop() {
            if let Some(next) = node.next.take() {
                q.push(Reverse(next));
            }
            p.next = Some(node);
            p = p.next.as_mut().unwrap();
        }

        head.next
    }
}

#[cfg(test)]
mod test {
    use super::Solution;
    use leetcode_prelude::{linkedlist, ListNode};

    #[test]
    fn test() {
        let input: Vec<Option<Box<ListNode>>> = vec![
            linkedlist![1, 4, 5],
            linkedlist![1, 3, 4],
            linkedlist![2, 6],
        ];
        let output = linkedlist![1, 1, 2, 3, 4, 4, 5, 6];
        assert_eq!(Solution::merge_k_lists(input), output);
    }
}

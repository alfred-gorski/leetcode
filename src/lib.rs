// #![feature(test)]
// #![feature(proc_macro_hygiene)]

// 不用 *, 否则智能提示会出现奇奇怪怪的问题
pub use leetcode_prelude::{linkedlist, btree, leetcode_test, vec_string, ListNode, TreeNode, assert_eq_sorted};

pub mod two_sum; //001 HashMap
pub mod is_match; //010 DP
pub mod first_missing_positive; //41 原地 Hash
pub mod reconstruct_itineary; 
pub mod lfu_cache; // 460 HashMap
pub mod get_max_repetitions; //466 
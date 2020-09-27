// #![feature(test)]
// #![feature(proc_macro_hygiene)]

// 不用 *, 否则智能提示会出现奇奇怪怪的问题
pub use leetcode_prelude::{
    assert_eq_sorted, btree, leetcode_test, linkedlist, vec_string, ListNode, TreeNode,
};

pub mod x0001two_sum; //HashMap
pub mod x0010is_match; //DP
pub mod x0041first_missing_positive; // 原地哈希
pub mod x0113path_sum;
pub mod x0125is_palindrome; // 双指针
pub mod x0201range_bitwise_and;
pub mod x0218get_skyline; //分治, 线段树, 扫描线法
pub mod x0332reconstruct_itineary; // 回溯
pub mod x0460lfu_cache; // HashMap
pub mod x0466get_max_repetitions;
pub mod x0501find_mode; // 二叉树 Morris 遍历
pub mod x0685find_redundant_directed_connection; //并查集
pub mod x0861matrix_score; // 贪心
pub mod x1109corp_flight_bookings; // 公交车算法
pub mod x1434number_ways; // DP

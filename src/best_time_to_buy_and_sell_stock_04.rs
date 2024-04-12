// link: https://leetcode.com/problems/best-time-to-buy-and-sell-stock/

use crate::Solution;

impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        let mut max_profit = 0;
        let mut price = prices[0];
        for i in 1..prices.len() {
            max_profit = i32::max(max_profit, prices[i] - price);
            price = i32::min(price, prices[i]);
        }
        max_profit
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_max_profit01() {
        assert_eq!(Solution::max_profit(vec![7, 1, 5, 3, 6, 4]), 5);
    }

    #[test]
    fn test_max_profit02() {
        assert_eq!(Solution::max_profit(vec![7, 6, 4, 3, 1]), 0);
    }
}

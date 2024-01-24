/// You are a professional robber planning to rob houses along a street.
/// Each house has a certain amount of money stashed, the only constraint stopping you from robbing each of them is that adjacent houses have security systems connected and it will automatically contact the police if two adjacent houses were broken into on the same night.
///
/// Given an integer array nums representing the amount of money of each house,
/// return the maximum amount of money you can rob tonight without alerting the police.
///
/// Example 1:
///
/// Input: nums = [1,2,3,1]
/// Output: 4
/// Explanation: Rob house 1 (money = 1) and then rob house 3 (money = 3).
/// Total amount you can rob = 1 + 3 = 4.
/// Example 2:
///
/// Input: nums = [2,7,9,3,1]
/// Output: 12
/// Explanation: Rob house 1 (money = 2), rob house 3 (money = 9) and rob house 5 (money = 1).
/// Total amount you can rob = 2 + 9 + 1 = 12.
///
/// Constraints:
///
/// 1 <= nums.length <= 100
/// 0 <= nums[i] <= 400
///
/// time O(n)
/// space O(n)
///
fn rob(nums: Vec<i32>) -> i32 {
    if nums.len() == 1 {
        return nums[0];
    }

    let mut dp = vec![0; nums.len()];

    for (i, house_money) in nums.into_iter().enumerate().rev() {
        let max_money =
            house_money + (std::cmp::max(dp.get(i + 2).unwrap_or(&0), dp.get(i + 3).unwrap_or(&0)));
        dp[i] = max_money;
    }

    std::cmp::max(dp[0], dp[1])
}

// time O(n)
// space O(1)
fn rob_2(nums: Vec<i32>) -> i32 {
    if nums.len() == 1 {
        return nums[0];
    }

    let mut rob_1 = 0;
    let mut rob_2 = 0;

    for (_, house_money) in nums.into_iter().enumerate().rev() {
        let max_money = std::cmp::max(rob_1, house_money + rob_2);
        rob_2 = rob_1;
        rob_1 = max_money;
    }

    rob_1
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rob() {
        assert_eq!(4, rob(vec![2, 1, 1, 2]));
        assert_eq!(1, rob(vec![1]));
        assert_eq!(4, rob(vec![1, 2, 3, 1]));
        assert_eq!(12, rob(vec![2, 7, 9, 3, 1]));

        assert_eq!(4, rob_2(vec![2, 1, 1, 2]));
        assert_eq!(1, rob_2(vec![1]));
        assert_eq!(4, rob_2(vec![1, 2, 3, 1]));
        assert_eq!(12, rob_2(vec![2, 7, 9, 3, 1]));
    }
}

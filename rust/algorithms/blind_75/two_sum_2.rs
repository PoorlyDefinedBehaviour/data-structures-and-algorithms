/// time (n)
/// space O(1)
pub fn two_sum_2(numbers: Vec<i32>, target: i32) -> Vec<i32> {
    let mut left = 0;
    let mut right = numbers.len() - 1;

    while left < right {
        let sum = numbers[left] + numbers[right];
        if sum == target {
            return vec![left as i32 + 1, right as i32 + 1];
        }

        if sum > target {
            right -= 1;
        } else {
            left += 1;
        }
    }

    unreachable!()
}

/// time O(n)
/// space O(1)
pub fn max_area(height: Vec<i32>) -> i32 {
    let mut left = 0;
    let mut right = height.len() - 1;

    let mut max = 0;

    while left < right {
        let lowest_height = std::cmp::min(height[left], height[right]);

        max = std::cmp::max(max, lowest_height * (right - left) as i32);

        if height[left] < height[right] {
            left += 1;
        } else {
            right -= 1;
        }
    }

    max
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(max_area(vec![1, 8, 6, 2, 5, 4, 8, 3, 7]), 49);
    }
}

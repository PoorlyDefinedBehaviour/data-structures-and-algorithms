/// time O(n)
/// space O(n)
fn product_except_self(nums: Vec<i32>) -> Vec<i32> {
    let mut suffix = vec![1; nums.len()];

    let mut product = 1;

    // time O(n)
    for i in (0..nums.len() - 1).rev() {
        product *= nums[i + 1];
        suffix[i] = product;
    }

    let mut prefix = 1;

    // time O(n)
    for i in 0..nums.len() {
        let product = prefix * suffix[i];
        prefix *= nums[i];
        suffix[i] = product;
    }

    suffix
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_product_except_self() {
        let cases = [
            (vec![1, 2, 3, 4], vec![24, 12, 8, 6]),
            (vec![-1, 1, 0, -3, 3], vec![0, 0, 9, 0, 0]),
        ];

        for (nums, expected) in cases {
            assert_eq!(expected, product_except_self(nums))
        }
    }
}

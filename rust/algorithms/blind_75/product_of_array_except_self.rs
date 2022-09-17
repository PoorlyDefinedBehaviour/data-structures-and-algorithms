/// time O(n)
/// space O(n)
pub fn product_except_self_two_arrays(nums: Vec<i32>) -> Vec<i32> {
    let mut left = vec![0; nums.len()];

    let mut right = vec![0; nums.len()];

    let mut i = 0;
    let mut j = nums.len() - 1;

    while i < nums.len() {
        left[i] = if i > 0 {
            left[i - 1] * nums[i]
        } else {
            nums[i]
        };

        right[j] = if j < nums.len() - 1 {
            right[j + 1] * nums[j]
        } else {
            nums[j]
        };

        i += 1;
        if j > 0 {
            j -= 1;
        }
    }

    let mut out = vec![0; nums.len()];

    for i in 0..nums.len() {
        out[i] = match i {
            0 => right[i + 1],
            i if i == nums.len() - 1 => left[i - 1],
            _ => left[i - 1] * right[i + 1],
        }
    }

    out
}

/// time O(n)
/// space O(1) (output does not count)
pub fn product_except_self(nums: Vec<i32>) -> Vec<i32> {
    let mut out = vec![1; nums.len()];

    let mut prefix = 1;
    for i in 0..nums.len() {
        out[i] = prefix;
        prefix *= nums[i];
    }

    let mut postfix = 1;
    for i in (0..nums.len()).rev() {
        out[i] *= postfix;
        postfix *= nums[i];
    }

    out
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let tests = vec![
            // (vec![1, 2, 3, 4], vec![24, 12, 8, 6])
            (vec![-1, 1, 0, -3, 3], vec![0, 0, 9, 0, 0]),
        ];

        for (input, expected) in tests {
            assert_eq!(expected, product_except_self(input));
        }
    }
}

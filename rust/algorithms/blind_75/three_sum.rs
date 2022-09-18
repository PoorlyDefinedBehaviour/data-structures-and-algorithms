/// time O(n^2)
/// space O(1)
pub fn three_sum(mut numbers: Vec<i32>) -> Vec<Vec<i32>> {
    numbers.sort_unstable();

    let mut triplets = Vec::new();

    for i in 0..numbers.len() {
        if i > 0 && numbers[i - 1] == numbers[i] {
            continue;
        }

        let mut left = i + 1;
        let mut right = numbers.len() - 1;
        while left < right {
            let sum = numbers[i] + numbers[left] + numbers[right];

            match sum {
                x if x > 0 => {
                    right -= 1;
                }
                x if x < 0 => {
                    left += 1;
                }
                _ => {
                    triplets.push(vec![numbers[i], numbers[left], numbers[right]]);
                    left += 1;
                    while left < numbers.len() && numbers[left] == numbers[left - 1] {
                        left += 1;
                    }
                }
            }
        }
    }

    triplets
}

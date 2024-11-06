use std::collections::HashSet;

/// Given an array of integers and a target value, determine the number of pairs of array elements that have a difference equal to the target value.
///
/// Example
///
/// There are three values that differ by : , , and . Return .
///
/// Function Description
///
/// Complete the pairs function below.
///
/// pairs has the following parameter(s):
///
/// int k: an integer, the target difference
/// int arr[n]: an array of integers
/// Returns
///
/// int: the number of pairs that satisfy the criterion
/// Input Format
///
/// The first line contains two space-separated integers  and , the size of  and the target value.
/// The second line contains  space-separated integers of the array .
///
/// Constraints
///
/// each integer  will be unique

fn pairs(k: i32, arr: &[i32]) -> i32 {
    let set: HashSet<_> = arr.iter().collect();

    let mut total = 0;

    for num in arr {
        let diff = (k - *num).abs();
        if set.contains(&diff) {
            total += 1;
        }
    }

    total
}

fn main() {
    let result = pairs(1, &[1, 2, 3, 4]);
    dbg!(&result);
}

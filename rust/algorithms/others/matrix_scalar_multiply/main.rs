fn multiply_scalar(mut matrix: Vec<Vec<i32>>, scalar: i32) -> Vec<Vec<i32>> {
    let rows = matrix.len();
    let columns = matrix[0].len();

    for i in 0..rows {
        for j in 0..columns {
            matrix[i][j] *= scalar;
        }
    }

    matrix
}

fn main() {
    let matrix = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];

    dbg!(multiply_scalar(matrix, 2));
}

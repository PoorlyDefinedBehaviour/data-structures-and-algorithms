fn transpose(matrix: &[Vec<i32>]) -> Vec<Vec<i32>> {
    let rows = matrix.len();
    let columns = matrix[0].len();

    let mut out = vec![vec![0; columns]; rows];

    for i in 0..rows {
        for j in 0..columns {
            out[i][j] = matrix[j][i];
        }
    }

    out
}

fn main() {
    let matrix = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];

    let transposed = transpose(&matrix);

    dbg!(matrix);

    // [
    //   [ 1, 4, 7, ],
    //   [ 2, 5, 8, ],
    //   [ 3, 6, 9, ],
    // ]
    dbg!(transposed);
}

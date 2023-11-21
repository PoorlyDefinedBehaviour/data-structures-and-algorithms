fn multiply(a: &[Vec<i32>], b: &[Vec<i32>]) -> Vec<Vec<i32>> {
    let a_rows = a.len();
    let a_columns = a[0].len();
    let b_columns = b[0].len();

    assert_eq!(a_rows, b_columns);

    let mut out = vec![vec![0; b_columns]; a_rows];

    for row_index in 0..a_rows {
        for column_index in 0..b_columns {
            let mut sum = 0;
            for k in 0..a_columns {
                sum += a[row_index][k] * b[k][column_index];
            }
            out[row_index][column_index] = sum;
        }
    }

    out
}

fn main() {
    let a = vec![vec![1, 2, 3], vec![4, 5, 6]];
    let b = vec![vec![7, 8], vec![9, 10], vec![11, 12]];

    let expected = vec![vec![58, 64], vec![139, 154]];

    let out = multiply(&a, &b);

    assert_eq!(expected, out);
}

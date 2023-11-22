fn add(a: &[Vec<i32>], b: &[Vec<i32>]) -> Vec<Vec<i32>> {
    let rows = a.len();
    let columns = a[0].len();

    assert_eq!(rows, b.len());
    assert_eq!(columns, b[0].len());

    let mut out = vec![vec![0; columns]; rows];

    for i in 0..rows {
        for j in 0..columns {
            out[i][j] = a[i][j] + b[i][j];
        }
    }

    out
}

fn main() {
    let a = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];

    let b = vec![vec![1, 1, 1], vec![1, 1, 1], vec![1, 1, 1]];

    dbg!(add(&a, &b));
}

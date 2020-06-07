/**
 * time O(2 * n * n) -> O(n)
 * space O(1)
 */
function rotateImage(matrix: number[][]): void {
  for (let i = 0; i < matrix.length; ++i) {
    for (let j = i; j < matrix.length; ++j) {
      const temp = matrix[i][j]
      matrix[i][j] = matrix[j][i]
      matrix[j][i] = temp
    }
  }

  for (let i = 0; i < matrix.length; ++i) {
    for (let j = 0; j < Math.floor(matrix.length / 2); ++j) {
      const temp = matrix[i][j]
      matrix[i][j] = matrix[i][matrix.length - 1 - j]
      matrix[i][matrix.length - 1 - j] = temp
    }
  }
}

const tap = (fn: Function) => <T>(x: T) => (fn(x), x)

function main() {

  console.log(
    tap(rotateImage)([
      [1, 4, 7],
      [2, 5, 8],
      [3, 6, 9]
    ])
  )
}
main()

/**
 * time O(n)
 * space O(1) numbers being returned don't count as extra space
 */
function spiralOrder(matrix: number[][]): number[] {
  const numbers: number[] = []

  let left = 0
  let right = matrix[0].length - 1
  let top = 0
  let bottom = matrix.length - 1

  const matrixSize = matrix.length * matrix[0].length

  while (numbers.length < matrixSize) {
    for (let i = left; i <= right; ++i) {
      numbers.push(matrix[left][i])
    }
    top++

    for (let i = top; i <= bottom; ++i) {
      numbers.push(matrix[i][right])
    }
    --right

    for (let i = right; i >= left; --i) {
      numbers.push(matrix[bottom][i])
    }
    --bottom

    for (let i = left; i <= right; ++i) {
      numbers.push(matrix[top][i])
    }
    ++left
  }

  return numbers
}

function main() {
  /*
  Given a matrix of m x n elements (m rows, n columns), return all elements of the matrix in spiral order.

  Example 1:

  Input:
  [
   [ 1, 2, 3 ],
   [ 4, 5, 6 ],
   [ 7, 8, 9 ]
  ]
  Output: [1,2,3,6,9,8,7,4,5]

  Example 2:

  Input:
  [ 
   [1, 2, 3, 4],
   [5, 6, 7, 8],
   [9,10,11,12]
  ]
  Output: [1,2,3,4,8,12,11,10,9,5,6,7]
  */
  console.log(
    spiralOrder([
      [1, 2, 3],
      [4, 5, 6],
      [7, 8, 9]
    ])
  )

  console.log(
    spiralOrder([
      [1, 2, 3, 4],
      [5, 6, 7, 8],
      [9, 10, 11, 12]
    ])
  )
}
main()

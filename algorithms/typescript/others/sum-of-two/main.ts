/**
 * time O(n + m) -> O(n)
 * space O(2n) -> O(n)
 */
function sumOfTwo(a: number[], b: number[], target: number): boolean {
  const complements = new Set(a.map(x => target - x))
  return b.some(x => complements.has(x))
}

function main() {
  /**
   * Given two arrays, A and B.
   * Find out if a number in A summed with a number in B equals a target C
   */
  console.log(sumOfTwo([0, 0, -5, 30212], [-10, 40, -3, 9], -8))
}
main()

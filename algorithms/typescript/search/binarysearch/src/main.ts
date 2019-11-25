/**
 * Best O(1)
 * Average O(log n)
 * Worst O(log n)
 * Space O(log n) recursive calls?
 */
const sum = x => y => x + y;
const divide = x => y => x / y;

function binary_search_impl<T>(
  array: T[],
  start: number,
  end: number,
  target: T
): number {
  if (start > end) return -1;

  const middle = Math.ceil(divide(sum(start)(end))(2));
  const element_found = array[middle];

  if (target === element_found) return middle;

  return target < element_found
    ? binary_search_impl(array, start, end - 1, target)
    : binary_search_impl(array, start + 1, end, target);
}

const binary_search = <T>(array: T[], target: T) =>
  binary_search_impl(array, 0, array.length - 1, target);

function main(): void {
  const array = [1, 2, 3, 4, 5];

  console.log(`binary_search(3) => ${binary_search(array, 3)}`);
  console.log(`binary_search(10) => ${binary_search(array, 10)}`);
  console.log(`binary_search(1) => ${binary_search(array, 1)}`);
  console.log(`binary_search(5) => ${binary_search(array, 5)}`);
}
main();

/**
 * Best O(1)
 * Average O(n)
 * Worst O(n)
 * Space O(1)
 */
function ordered_linear_search<T>(array: T[], target: T): number {
  for (let i = 0; i < array.length; ++i) {
    if (array[i] === target) return i;
    if (array[i] > target) return -1;
  }

  return -1;
}

function main(): void {
  const array = [1, 2, 3, 4, 5];

  console.log(`ordered_linear_search(3) => ${ordered_linear_search(array, 3)}`);
  console.log(
    `ordered_linear_search(10) => ${ordered_linear_search(array, 10)}`
  );
  console.log(`ordered_linear_search(1) => ${ordered_linear_search(array, 1)}`);
  console.log(`ordered_linear_search(5) => ${ordered_linear_search(array, 5)}`);
  console.log(
    `ordered_linear_search(-1) => ${ordered_linear_search(array, -1)}`
  );
}
main();

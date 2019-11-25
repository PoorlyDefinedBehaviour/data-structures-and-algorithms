/**
 * Best O(1)
 * Average O(n)
 * Worst O(n)
 * Space O(1)
 */
function linear_search<T>(array: T[], target: T): number {
  for (let i = 0; i < array.length; ++i) if (array[i] === target) return i;

  return -1;
}

function main(): void {
  const array = [1, 2, 3, 4, 5];

  console.log(`linear_search(3) => ${linear_search(array, 3)}`);
  console.log(`linear_search(10) => ${linear_search(array, 10)}`);
  console.log(`linear_search(1) => ${linear_search(array, 1)}`);
  console.log(`linear_search(5) => ${linear_search(array, 5)}`);
}
main();

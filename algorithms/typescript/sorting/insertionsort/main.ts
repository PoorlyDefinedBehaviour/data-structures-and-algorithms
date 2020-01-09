import { tap } from "../std/Std.lib";

/**
 * Best O(n)
 * Average O(n^2)
 * Worst O(n^2)
 * Space O(1)
 */
function insertion_sort<T>(array: T[]): void {
  for (let i = 1; i < array.length; ++i) {
    let j = i;

    while (j > 0 && array[j - 1] > array[j]) {
      const temp = array[j];
      array[j] = array[j - 1];
      array[j - 1] = temp;
      j -= 1;
    }
  }
}

function main(): void {
  console.log(tap(insertion_sort)([5, 4, 3, 2, 1]));
}
main();

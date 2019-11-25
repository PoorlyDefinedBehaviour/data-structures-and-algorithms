import { tap } from "../../std/Std.lib";

/**
 * Best O(n)
 * Average O(n^2)
 * Worst O(n^2)
 * Space O(1)
 */
function bubble_sort<T>(array: T[]): void {
  for (let i = 0; i < array.length - 1; ++i) {
    let swapped = false;

    for (let j = i + 1; j < array.length; ++j) {
      if (array[i] > array[j]) {
        const temp = array[i];
        array[i] = array[j];
        array[j] = temp;
        swapped = true;
      }
    }

    if (!swapped) break;
  }
}

function main(): void {
  console.log(tap(bubble_sort)([2, 1, 3, 4, 5, -1]));
}
main();

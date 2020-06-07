import { tap } from "../std/Std.lib";

/**
 * Best O(n^2)
 * Average O(n^2)
 * Worst O(n^2)
 * Space O(1)
 */
function selection_sort<T>(array: T[]): void {
  for (let i = 0; i < array.length; ++i) {
    let smallest = i;

    for (let j = i + 1; j < array.length; ++j) {
      if (array[j] < array[smallest]) {
        smallest = j;
      }
    }

    const temp = array[i];
    array[i] = array[smallest];
    array[smallest] = temp;
  }
}

function main(): void {
  console.log(tap(selection_sort)([5, 4, 3, 2, 1]));
}
main();

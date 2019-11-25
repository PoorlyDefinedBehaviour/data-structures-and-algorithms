import { tap } from "../../std/Std.lib";

/**
 * Best O(n log n)
 * Average O(n log n)
 * Worst O(n^2)
 * Space O(log n)
 */
function quick_sort_impl<T>(array: T[], start: number, end: number): void {
  if (start >= end) return;

  let pivot_index = start;

  const pivot_value = array[end];

  for (let i = start; i < end; ++i) {
    if (array[i] < pivot_value) {
      const temp = array[i];
      array[i] = array[pivot_index];
      array[pivot_index++] = temp;
    }
  }

  const temp = array[pivot_index];
  array[pivot_index] = array[end];
  array[end] = temp;

  quick_sort_impl(array, start, pivot_index - 1);
  quick_sort_impl(array, pivot_index + 1, end);
}

const quick_sort = <T>(array: T[]) =>
  quick_sort_impl(array, 0, array.length - 1);

function main(): void {
  console.log(tap(quick_sort)([5, 4, 3, 2, 1]));
}
main();

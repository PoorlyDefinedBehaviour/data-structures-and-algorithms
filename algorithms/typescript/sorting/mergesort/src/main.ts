/**
 * Best O(n log n)
 * Average O(n log n)
 * Worst O(n log n)
 * Space O(n)
 */
function merge_sort<T>(array: T[]): T[] {
  if (array.length <= 1) return array;

  const equal: T[] = [];
  const smaller: T[] = [];
  const bigger: T[] = [];

  const pivot = array[Math.floor(array.length / 2)];

  array.forEach(x => {
    if (x < pivot) smaller.push(x);
    else if (x > pivot) bigger.push(x);
    else equal.push(x);
  });

  return [...merge_sort(smaller), ...equal, ...merge_sort(bigger)];
}

function main(): void {
  const array = [5, 4, 8, -10, 3, 2, 1];

  console.log("unsorted array =>", array);
  console.log("sorted array =>", merge_sort(array));
}
main();

const reverse = foldl((xs, x) => [x].concat(xs))([]);

const takeWhile = (predicate) => (xs) =>
  predicate(head(xs)) ? [head(xs)].concat(takeWhile(predicate)(tail(xs))) : [];

const dropWhile = (predicate) => (xs) =>
  predicate(head(xs)) ? dropWhile(predicate)(tail(xs)) : xs;

const double = (x) => x * 2;

const isEven = (x) => (x & 1) === 0;

console.log("map(double)([1,2,3])", map(double)([1, 2, 3]));
console.log("filter(isEven)([1,2,3])", filter(isEven)([1, 2, 3]));
console.log("reverse([1,2,3])", reverse([1, 2, 3]));
console.log("takeWhile(isEven)([1,2,3])", takeWhile(isEven)([1, 2, 3]));
console.log("dropWhile(isEven)([1,2,3])", dropWhile(isEven)([1, 2, 3]));

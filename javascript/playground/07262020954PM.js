const fold = (f) => (initialValue) => (xs) => xs.reduce(f, initialValue);

const binary = (op) => (x, y) => eval(`${x} ${op} ${y}`);

const mult = binary("*");

const add = binary("+");

const product = fold(mult)(1);

const inc = (x) => add(1, x);

const length = fold(inc)(0);

const reverse = fold((xs, x) => [x].concat(xs))([]);

const replicate = (length) => (x) => Array.from({ length }).map(() => x);

const nth = (index) => (xs) => {
  const doNth = (currentIndex) =>
    currentIndex === index ? xs[currentIndex] : doNth(currentIndex + 1);

  return doNth(index);
};

const elem = (x) => (xs) => {
  const doElem = (index) =>
    xs[index] === x || (index < xs.length && doElem(index + 1));

  return doElem(0);
};

console.log("product([1, 2, 3]", product([1, 2, 3]));
console.log("length([1, 2, 3]", length([1, 2, 3]));
console.log("reverse([1, 2, 3]", reverse([1, 2, 3]));
console.log("replicate(3)(3)", replicate(3)(3));
console.log("nth(2)([1, 2, 3])", nth(2)([1, 2, 3]));
console.log("elem(2)([1, 2, 3])", elem(2)([1, 2, 3]));

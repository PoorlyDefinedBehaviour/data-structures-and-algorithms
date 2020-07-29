const Just = (value) => ({
  __tag: "Just",
  value,
  map: (f) => Just(f(value)),
  bind: (f) => f(value),
  fold: (_, g) => g(value),
});

const Nothing = () => ({
  __tag: "Nothing",
  map: (_f) => Nothing(),
  bind: (_f) => Nothing(),
  fold: (f, _) => f(),
});

const fmap = (f) => (functor) => functor.map(f);

const double = (x) => x * 2;

const head = (xs) => (xs.length > 0 ? Just(xs[0]) : Nothing());

const x = head([[1, 2, 3]]);
const y = x.bind(head);
const z = fmap(double)(y);
console.log(z);

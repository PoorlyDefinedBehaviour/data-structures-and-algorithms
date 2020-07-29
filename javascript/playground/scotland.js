const sequence = (...functions) =>
  functions.reduce((promise, fn) => promise.then(fn), Promise.resolve());

const Right = (value) => ({
  __tag: "Right",
  value,
  map: (f) => Right(f(value)),
  bind: (f) => f(value),
  fold: (_, f) => f(value),
});

const Left = (error) => ({
  __tag: "Left",
  error,
  map: (f) => Left(error),
  bind: (f) => Left(error),
  fold: (f, _) => f(error),
});

const fmap = (f) => (functor) => functor.map(f);

const bind = (f) => (monad) => monad.bind(f);

const fold = (f, g) => (foldable) => foldable.fold(f, g);

const divide = (a) => (b) =>
  b === 0 ? Left("Tried to divide ${a} by ${b}") : Right(a / b);

const multiply = (a) => (b) => a * b;

const sum = (a) => (b) => a + b;

const main = async () => {
  const result = await sequence(
    () => Promise.resolve(divide(32)(2)),
    bind(divide(32)),
    fmap(multiply(2)),
    fmap(sum(12)),
    fold(
      (_error) => "Oops",
      (value) => `Yay -> ${value}`
    )
  );

  console.log(result);
};

main();

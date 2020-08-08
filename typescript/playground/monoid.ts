import * as R from "ramda"

type Monoid<A> = {
  empty: A
  concat: (a: A, b: A) => A
}

const addition: Monoid<number> = {
  empty: 0,
  concat: (a, b) => a + b,
}

const subtraction: Monoid<number> = {
  empty: 0,
  concat: (a, b) => a - b,
}

const multiplication: Monoid<number> = {
  empty: 1,
  concat: (a, b) => a * b,
}

const foldMap = <A, B>(monoid: Monoid<B>, f: (a: A) => B, xs: A[]) =>
  xs.reduce((accumulator, x) => monoid.concat(accumulator, f(x)), monoid.empty)

foldMap(addition, R.add(1), [1, 2, 3]) |> console.log
foldMap(subtraction, R.add(1), [1, 2, 3]) |> console.log
foldMap(multiplication, R.add(1), [1, 2, 3]) |> console.log

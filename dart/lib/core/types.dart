typedef Predicate<T> = bool Function(T value);

typedef Morphism<T, U> = U Function(T value);

typedef Reducer<T, U> = U Function(U accumulator, T value);

typedef Consumer<T> = void Function(T value);

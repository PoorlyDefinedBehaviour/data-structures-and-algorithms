import 'package:equatable/equatable.dart';
import 'package:snowblind/core/types.dart';

abstract class ConsList<T> {
  int length();
  ConsList<U> map<U>(final Morphism<T, U> morphism);
  ConsList<T> filter<U>(final Predicate<T> morphism);
  ConsList<U> flatMap<U>(final Morphism<T, ConsList<U>> morphism);
}

class Cons<T> extends Equatable implements ConsList<T> {
  final T value;

  final ConsList<T> next;

  const Cons(
    this.value,
    this.next,
  );

  @override
  List<Object> get props => [value, next];

  @override
  int length() => 1 + (next is Cons<T> ? (next as Cons<T>).length() : 0);

  @override
  ConsList<T> filter<U>(final Predicate<T> predicate) => predicate(value)
      ? Cons(value, next.filter(predicate))
      : next.filter(predicate);

  @override
  ConsList<U> flatMap<U>(final Morphism<T, ConsList<U>> morphism) =>
      map((x) => (morphism(x) as Cons<U>).value);

  @override
  Cons<U> map<U>(final Morphism<T, U> morphism) =>
      Cons(morphism(value), next.map(morphism));
}

class Nil<T> extends Equatable implements ConsList<T> {
  @override
  List<Object> get props => [];

  @override
  int length() => 0;

  @override
  ConsList<U> map<U>(Morphism<T, U> morphism) => Nil();

  @override
  ConsList<T> filter<U>(final Predicate<T> predicate) => Nil();

  @override
  ConsList<U> flatMap<U>(final Morphism<T, ConsList<U>> morphism) => Nil();
}

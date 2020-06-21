import 'package:equatable/equatable.dart';
import 'package:snowblind/core/types.dart';

abstract class Maybe<T> {
  Maybe<U> map<U>(final Morphism<T, U> morphism);
  Maybe<U> flatMap<U>(final Morphism<T, Maybe<U>> morphism);
  U fold<U>(final Producer<U> onNothing, final Morphism<T, U> onJust);
}

class Just<T> extends Equatable implements Maybe<T> {
  final T value;

  Just(this.value);

  @override
  List<Object> get props => [value];

  Maybe<U> map<U>(final Morphism<T, U> morphism) => Just(morphism(value));

  Maybe<U> flatMap<U>(final Morphism<T, Maybe<U>> morphism) => morphism(value);

  @override
  U fold<U>(onNothing, onJust) => onJust(value);
}

class Nothing extends Equatable implements Maybe<void> {
  @override
  Maybe<U> map<U>(final Morphism<dynamic, U> morphism) => Nothing() as Maybe<U>;

  @override
  List<Object> get props => [];

  @override
  Maybe<U> flatMap<U>(final Morphism<dynamic, Maybe<U>> morphism) =>
      Nothing() as Maybe<U>;

  @override
  U fold<U>(final Producer<U> onNothing, final Morphism<dynamic, U> onJust) =>
      onNothing();
}

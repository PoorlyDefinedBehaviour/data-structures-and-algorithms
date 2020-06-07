import 'package:dartz/dartz.dart';
import 'package:equatable/equatable.dart';
import 'package:snowblind/core/types.dart';

class _Node<T> extends Equatable {
  final T value;
  _Node<T> next = null;

  _Node(this.value);

  @override
  List<Object> get props => [value];
}

class LinkedList<T> extends Equatable {
  int _length = 0;
  _Node<T> _head = null;
  _Node<T> _tail = null;

  int get length => _length;

  @override
  List<Object> get props => [_length, _head, _tail];

  Option<T> head() => _head != null ? Some(_head.value) : None();

  Option<T> tail() => _tail != null ? Some(_tail.value) : None();

  LinkedList<T> push(final T value) {
    _length += 1;

    if (_tail == null) {
      _tail = _Node(value);
      _head = _tail;

      return this;
    }

    _tail.next = _Node(value);
    _tail = _tail.next;

    return this;
  }

  void forEach(Consumer<T> consumer) {
    var currentNode = _head;

    while (currentNode != null) {
      consumer(currentNode.value);

      currentNode = currentNode.next;
    }
  }

  U reduce<U>(final U initialValue, final Reducer<T, U> reducer) {
    var result = initialValue;

    var currentNode = _head;

    while (currentNode != null) {
      result = reducer(result, currentNode.value);

      currentNode = currentNode.next;
    }

    return result;
  }

  LinkedList<T> filter(final Predicate<T> predicate) {
    final list = LinkedList<T>();

    var currentNode = _head;

    while (currentNode != null) {
      if (predicate(currentNode.value)) {
        list.push(currentNode.value);
      }

      currentNode = currentNode.next;
    }

    return list;
  }

  LinkedList<U> map<U>(final Morphism<T, U> morphism) {
    final list = LinkedList<U>();

    var currentNode = _head;

    while (currentNode != null) {
      list.push(morphism(currentNode.value));

      currentNode = currentNode.next;
    }

    return list;
  }

  LinkedList<T> concat(final LinkedList<T> other) {
    var list = LinkedList<T>();

    var thisListCurrentNode = _head;

    while (thisListCurrentNode != null) {
      list.push(thisListCurrentNode.value);

      thisListCurrentNode = thisListCurrentNode.next;
    }

    var otherListCurrentNode = other._head;

    while (otherListCurrentNode != null) {
      list.push(otherListCurrentNode.value);

      otherListCurrentNode = otherListCurrentNode.next;
    }

    return list;
  }
}

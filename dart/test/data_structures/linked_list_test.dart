import 'package:dartz/dartz.dart';
import 'package:mockito/mockito.dart';
import 'package:snowblind/data_structures/linked_list.dart';
import 'package:test/test.dart';

abstract class Callable<T, U> {
  U call(final T value);
}

class MockForEachFunction<T, U> extends Mock implements Callable<T, U> {}

void main() {
  group("LinkedList test suite", () {
    group('head()', () {
      test('when the list is empty, should return None', () {
        final list = LinkedList<int>();

        expect(list.head(), None<int>());
      });

      test('when the list is not empty, should return the first element', () {
        final list = LinkedList<int>().push(10);

        expect(list.head(), equals(Some(10)));
      });
    });

    group('tail()', () {
      test('when the list is empty, should return None', () {
        final list = LinkedList<int>();

        expect(list.tail(), None<int>());
      });

      test('when the list is not empty, should return the first element', () {
        final list = LinkedList<int>().push(10);

        expect(list.tail(), equals(Some(10)));
      });
    });

    group('length', () {
      test("when the list is empty, should return 0", () {
        final list = LinkedList<String>();

        expect(list.length, equals(0));
      });

      test("when an element is added, should increase by 1", () {
        final list = LinkedList<String>();

        expect(list.length, equals(0));

        list.push("hello");

        expect(list.length, equals(1));

        list.push("world");

        expect(list.length, equals(2));
      });
    });

    group("filter()", () {
      test("when called on an empty list, should return a new empty list", () {
        final list = LinkedList<bool>();

        final result = list.filter((_) => true);

        expect(result, equals(LinkedList<bool>()));
      });

      test(
          "when called on a non empty list, should returns a new list with the elements that passed the predicate",
          () {
        final list = LinkedList<int>().push(1).push(2).push(3).push(4).push(5);

        final expected = LinkedList<int>().push(2).push(4);

        final isEven = (x) => x & 1 == 0;

        final result = list.filter(isEven);

        expect(result, equals(expected));
      });
    });

    group("map()", () {
      test("when called on an empty list, should return a new empty list", () {
        final list = LinkedList<int>();

        final result = list.map((x) => x + 1);

        expect(result, equals(LinkedList<int>()));
      });

      test(
          "when called on an empty list, should return a new list with the values return by the provided function",
          () {
        final list = LinkedList<int>().push(1).push(2).push(3);

        final expected = LinkedList<int>().push(2).push(4).push(6);

        final result = list.map((x) => x * 2);

        expect(result, equals(expected));
      });
    });

    group("reduce()", () {
      test("when called on an empty list should return the initial value", () {
        final list = LinkedList<int>();

        final sum = (x, y) => x + y;

        expect(list.reduce(0, sum), equals(0));
      });

      test("when called on a non empty list should return the reduced value",
          () {
        final list = LinkedList<int>().push(1).push(2).push(3);

        final sum = (x, y) => x + y;

        expect(list.reduce(0, sum), equals(6));
      });
    });

    group("concat()", () {
      test("when called with two empty lists, should return a new empty list",
          () {
        final emptyListA = LinkedList<int>();

        final emptyListB = LinkedList<int>();

        final result = emptyListA.concat(emptyListB);

        expect(result, equals(LinkedList<int>()));
      });

      test(
          "when called on an empty list should return a new list with whatever elements the other list has",
          () {
        final emptyList = LinkedList<int>();

        final nonEmptyList = LinkedList<int>().push(1).push(2).push(3);

        final expected = LinkedList<int>().push(1).push(2).push(3);

        final result = emptyList.concat(nonEmptyList);

        expect(result, equals(expected));
      });

      test(
          "when called with an empty list should return a new list with whatever elements the first list has",
          () {
        final nonEmptyList = LinkedList<int>().push(1).push(2).push(3);

        final emptyList = LinkedList<int>();

        final expected = LinkedList<int>().push(1).push(2).push(3);

        final result = nonEmptyList.concat(emptyList);

        expect(result, equals(expected));
      });
    });

    group("forEach()", () {
      test("when the list is empty, should have no effect", () {
        final list = LinkedList<String>();

        final mockForEachFunction = MockForEachFunction<String, void>();

        list.forEach(mockForEachFunction.call);

        verifyNever(mockForEachFunction.call(any));
      });

      test(
          "when the list is not empty empty, should call the consumer with each element of the list",
          () {
        final list = LinkedList<String>().push("hello").push("world").push("!");

        final mockForEachFunction = MockForEachFunction<String, void>();

        list.forEach(mockForEachFunction.call);

        verify(mockForEachFunction.call("hello")).called(1);
        verify(mockForEachFunction.call("world")).called(1);
        verify(mockForEachFunction.call("!")).called(1);
      });
    });
  });
}

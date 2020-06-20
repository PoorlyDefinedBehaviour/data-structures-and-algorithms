import 'package:snowblind/data_structures/cons_list.dart';
import 'package:test/test.dart';

void main() {
  group("Cons list test suite", () {
    group("when the list is empty", () {
      test("list.length() should return 0", () {
        final list = Nil();

        expect(list.length(), equals(0));
      });

      test("list.map(f) should return an empty list", () {
        final list = Nil();

        expect(list.map((x) => x * x), equals(Nil()));
      });

      test("list.filter(f) should return an empty list", () {
        final list = Nil();

        expect(list.filter((x) => x & 1 == 0), equals(Nil()));
      });

      test("list.flatMap(f) should return an empty list", () {
        final list = Nil();

        expect(list.flatMap((x) => x * 2), equals(Nil()));
      });
    });

    group("when the list is not empty", () {
      test("list.length() should return a positive integer", () {
        ConsList<int> list = Cons(10, Nil());

        expect(list.length(), equals(1));
      });

      test(
          "list.filter(f) should return a list with values that passed the predicated",
          () {
        ConsList<int> list = Cons(
            1,
            Cons(
                2,
                Cons(
                    3,
                    Cons(
                        4,
                        Cons(
                          5,
                          Nil(),
                        )))));

        final isEven = (x) => x & 1 == 0;

        ConsList<int> expected = Cons(2, Cons(4, Nil()));

        expect(list.filter(isEven), equals(expected));
      });

      test("list.map(f) should return a list with mapped values", () {
        ConsList<int> list = Cons(
          1,
          Cons(
            2,
            Cons(
              3,
              Nil(),
            ),
          ),
        );

        int Function(int) square = (x) => x * x;

        ConsList<int> expected = Cons(1, Cons(4, Cons(9, Nil())));

        expect(list.map(square), equals(expected));
      });

      test("list.flatMap(f) should return a list with flattened values", () {
        ConsList<int> list = Cons(
          1,
          Cons(
            2,
            Cons(
              3,
              Nil(),
            ),
          ),
        );

        ConsList<int> Function(int) square = (x) => Cons(x * x, Nil());

        ConsList<int> expected = Cons(1, Cons(4, Cons(9, Nil())));

        expect(list.flatMap(square), equals(expected));
      });
    });
  });
}

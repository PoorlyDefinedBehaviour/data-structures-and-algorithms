import 'package:snowblind/data_structures/maybe.dart';
import 'package:test/test.dart';

void main() {
  group("maybe test suite", () {
    group("Just", () {
      test(".map(f) should return a Just with the mapped value", () {
        final maybe = Just(10);

        expect(maybe.map((x) => x * 2), equals(Just(20)));
      });

      test(".flatMap(f) should return a Just with the mapped value flattened",
          () {
        final maybe = Just(10);

        expect(
          maybe.flatMap((x) => Just(x * 2)),
          equals(Just(20)),
        );
      });

      test(
          ".fold should call onJust function with the value thats held by the maybe",
          () {
        final maybe = Just(2);

        final result = maybe.fold(() => 40, (x) => x * 2);

        expect(result, equals(4));
      });
    });

    group("Nothing", () {
      test(".map(f) should return Nothing", () {
        final maybe = Nothing();

        expect(maybe.map((x) => x * 2), Nothing());
      });

      test(".flatMap(f) should return Nothing", () {
        final maybe = Nothing();

        expect(
          maybe.flatMap((x) => Just(x * 2)),
          equals(Nothing()),
        );
      });

      test(".fold should call onNothing function", () {
        final maybe = Nothing();

        final result = maybe.fold(() => 40, (x) => x * 2);

        expect(result, equals(40));
      });
    });
  });
}

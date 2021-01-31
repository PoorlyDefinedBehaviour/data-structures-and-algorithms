(ns main(ns kata.core)
(use 'clojure.test)

(defn fib [x]
  (if (<= x 1)
    x
    (+ (fib (dec x)) (fib (dec (dec x))))))

(deftest fib-tests
  (is (= 0 (fib 0)))
  (is (= 1 (fib 1)))
  (is (= 1 (fib 2)))
  (is (= 2 (fib 3)))
  (is (= 3 (fib 4)))
  (is (= 5 (fib 5)))
  (is (= 8 (fib 6)))
  (is (= 13 (fib 7))))

(run-all-tests))

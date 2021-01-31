(ns kata.core)
(use 'clojure.test)

(defn fib [x]
  (if (<= x 1)
    x
    (* x (fib (dec x)))))


(deftest fib-tests
  (is (= 120 (fib 5)))
  (is (= 24 (fib 4)))
  (is (= 6 (fib 3)))
  (is (= 0 (fib 0)))
  (is (= -1 (fib -1))))

(run-all-tests)
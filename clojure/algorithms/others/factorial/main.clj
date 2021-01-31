(ns kata.core)
(use 'clojure.test)

(defn factorial [x]
  (if (<= x 1)
    x
    (* x (factorial (dec x)))))


(deftest factorial-tests
  (is (= 120 (factorial 5)))
  (is (= 24 (factorial 4)))
  (is (= 6 (factorial 3)))
  (is (= 0 (factorial 0)))
  (is (= -1 (factorial -1))))

(run-all-tests)
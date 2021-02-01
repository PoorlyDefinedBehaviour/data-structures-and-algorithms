(ns kata.core)
(use 'clojure.test)

(defn True [then _else] then)

(defn False [_then else] else)

(defn If [condition then else]
  (condition then else))

(defn Not [bool]
  (If bool False True))

(defn Or [condition-a condition-b]
  (If condition-a True condition-b))

(defn And [condition-a condition-b]
  (If condition-a condition-b False))

(deftest church-encoding-tests
  (is (= 1 (If (Not False) 1 2)))
  (is (= 1 (If (And True True) 1 2)))
  (is (= 2 (If (Or True True) 2 1)))
  (is (= 2 (If (And True False) 1 2))))

(run-tests)
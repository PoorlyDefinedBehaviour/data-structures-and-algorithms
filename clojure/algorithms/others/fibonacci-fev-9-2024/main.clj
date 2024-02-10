(ns core
  (:require [clojure.test :refer [deftest testing is]]))

;; The Fibonacci numbers, commonly denoted F(n) form a sequence, 
;; called the Fibonacci sequence, such that each number is the sum of the two preceding ones, 
;; starting from 0 and 1. That is,
;; 
;; F(0) = 0, F(1) = 1
;; F(n) = F(n - 1) + F(n - 2), for n > 1.
;; Given n, calculate F(n).
;; 
;; Example 1:
;; 
;; Input: n = 2
;; Output: 1
;; Explanation: F(2) = F(1) + F(0) = 1 + 0 = 1.
;; Example 2:
;; 
;; Input: n = 3
;; Output: 2
;; Explanation: F(3) = F(2) + F(1) = 1 + 1 = 2.
;; Example 3:
;; 
;; Input: n = 4
;; Output: 3
;; Explanation: F(4) = F(3) + F(2) = 2 + 1 = 3.
;; 
;; Constraints:
;; 
;; 0 <= n <= 30

;; time 2^n
;; space O(n)
(defn fib [n]
  (case n
    0 0
    1 1
    (+ (fib (- n 1)) (fib (- n 2)))))

;; time O(n)
;; space O(n)
(defn fib-memo [n]
  (let [cache (atom {})]
    (letfn [(go [n]
                (case n
                  0 0
                  1 1
                  (if (contains? @cache n)
                    (get @cache n)
                    (let [result (+ (go (- n 1)) (go (- n 2)))]
                      (swap! cache assoc n result)
                      result))))]
      (go n))))

;; time O(n)
;; space O(1)
(defn fib-dynamic [n]
  (loop [a 1
         b 0
         n n]
    (if (<= n  0)
      b
      (let [result (+ a b)]
        (recur result a (dec n))))))

(defmacro test-function [f]
  (let [function-name (gensym (name f))]
    `(deftest
       ~function-name
       (testing " example 1"
         (is (= 1 (~f 2))))
       (testing " example 2"
         (is (= 2 (~f 3))))
       (testing " example 3"
         (is (= 3 (~f 4)))))))

(test-function fib)
(test-function fib-memo)
(test-function fib-dynamic)
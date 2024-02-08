(ns core
  (:require [clojure.test :refer [deftest testing is]]))

;; You are climbing a staircase. It takes n steps to reach the top.

;; Each time you can either climb 1 or 2 steps. In how many distinct ways can you climb to the top?

;; Example 1:

;; Input: n = 2
;; Output: 2
;; Explanation: There are two ways to climb to the top.
;; 1. 1 step + 1 step
;; 2. 2 steps
;; Example 2:

;; Input: n = 3
;; Output: 3
;; Explanation: There are three ways to climb to the top.
;; 1. 1 step + 1 step + 1 step
;; 2. 1 step + 2 steps
;; 3. 2 steps + 1 step

;; Constraints:

;; 1 <= n <= 45

;; time O(n)
;; space O(1)
(defn climb-stairs-bottom-up [top]
  (loop [n top
         n-plus-1 0
         n-plus-2 1]
    (let [ways (+ n-plus-1 n-plus-2)]
      (if (> n 0)
        (recur (dec n) ways n-plus-1)
        ways))))

(defmacro test-function [f]
  (let [function-name (gensym (name f))]
    `(deftest
       ~function-name
       (testing " example 1"
         (is (= 2 (~f 2))))
       (testing " example 2"
         (is (= 3 (~f 3))))
       (testing " example 3"
         (is (= 8 (~f 5)))))))

(test-function climb-stairs-bottom-up)
(ns core
  (:require [clojure.test :refer [deftest testing is]]))

;; You are climbing a staircase. It takes n steps to reach the top.
;; 
;; Each time you can either climb 1 or 2 steps. In how many distinct ways can you climb to the top?
;; 
;; Example 1:
;; 
;; Input: n = 2
;; Output: 2
;; Explanation: There are two ways to climb to the top.
;; 1. 1 step + 1 step
;; 2. 2 steps
;; 
;; Example 2:
;; 
;; Input: n = 3
;; Output: 3
;; 
;; Explanation: There are three ways to climb to the top.
;; 1. 1 step + 1 step + 1 step
;; 2. 1 step + 2 steps
;; 3. 2 steps + 1 step
;;  
;; Constraints:
;; 
;; 1 <= n <= 45

;; time O(2^n)
;; space O(n)
(defn climb-stairs-dfs [n]
  (letfn [(go [steps]
              (cond
                (= steps n) 1
                (> steps n) 0
                :else (+ (go (+ steps 1)) (go (+ steps 2)))))]
    (go 0)))

(defn my-memoize [f]
  (let [cache (atom {})]
    (fn [x]
      (if (contains? @cache x)
        (get @cache x)
        (let [value (f x)]
          (swap! cache assoc x value)
          value)))))

;; time O(n)
;; space O(n)
(defn climb-stairs-dfs-memoize [n]
  #_{:clj-kondo/ignore [:inline-def :uninitialized-var]}
  (def go)
  (let [go (my-memoize (fn [steps]
                         (cond
                           (= steps n) 1
                           (> steps n) 0
                           :else (+ (go (+ steps 1)) (go (+ steps 2))))))]
    (go 0)))

;; I'm aware that we only need to keep every step.
;; time O(n)
;; space O(n)
(defn climb-stairs-dynamic [n]
  (letfn [(go [steps already-computed]
              (if (= steps n)
                (get already-computed (- steps 1))
                (let [ways-to-get-to-current-step (get already-computed (- steps 1))
                      steps-plus-one (+ steps 1)
                      steps-plus-two (+ steps 2)
                      new-steps (+ ways-to-get-to-current-step (if (<= steps-plus-one n) 1 0) (if (<= steps-plus-two n) 1 0))]
                  (recur (+ steps 1) (assoc already-computed steps new-steps)))))]
    (go 0 {})))

;; time O(n)
;; space O(n)
;; Start computing the number of steps to get to the target starting from the target.
;; Could keep only the last 2 values to use less memory.
(defn climb-stairs-dynamic-bottom-up [n]
  (letfn [(go [step already-computed]
              (let [ways-to-get-to-target (if (= step n)
                                            1
                                            (+ (get already-computed (+ step 1) 0) (get already-computed (+ step 2) 0)))]
                (if (= step 0)
                  ways-to-get-to-target
                  (recur (- step 1) (assoc already-computed step ways-to-get-to-target)))))]
    (go 0 {})))

;; time O(n)
;; space O(1)
;; Same strategy as climb-stairs-dynamic-bottom-up but keeping only the last 2 number of ways to get to the target.
(defn climb-stairs-dynamic-bottom-up-2 [n]
  (letfn [(go [step already-computed]
              (let [ways-to-get-to-target (if (= step n)
                                            1
                                            (+ (get already-computed 0) (get already-computed 1)))
                    previous-ways-to-get-to-target (get already-computed 0)
                    new-already-computed (-> already-computed
                                             (assoc 0 ways-to-get-to-target)
                                             (assoc 1 previous-ways-to-get-to-target))]
                (if (= step 0)
                  ways-to-get-to-target
                  (recur (- step 1) new-already-computed))))]
    (go 0 [0 0])))

(defmacro test-function [f]
  (let [function-name (gensym (name f))]
    `(deftest
       ~function-name
       (testing "example 1"
         (is (= 2 (climb-stairs-dfs 2))))
       (testing "example 2"
         (is (= 3 (climb-stairs-dfs 3))))
       (testing "example 3"
         (is (= 8 (climb-stairs-dfs 5)))))))

(test-function climb-stairs-dfs)
(test-function climb-stairs-dfs-memoize)
(test-function climb-stairs-dynamic)
(test-function climb-stairs-dynamic-bottom-up)
(test-function climb-stairs-dynamic-bottom-up-2)
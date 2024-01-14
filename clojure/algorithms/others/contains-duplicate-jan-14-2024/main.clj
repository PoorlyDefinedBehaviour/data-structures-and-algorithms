(ns core
  (:require [clojure.test :refer [deftest testing is]]))

;; Given an integer array nums, return true if any value appears at least twice in the array, and return false if every element is distinct.
;; 
;; Example 1:
;; 
;; Input: nums = [1,2,3,1]
;; Output: true
;; Example 2:
;; 
;; Input: nums = [1,2,3,4]
;; Output: false
;; Example 3:
;; 
;; Input: nums = [1,1,1,3,3,4,3,2,4,2]
;; Output: true

(defn contains-duplicate [nums]
  (letfn [(go [num nums seen]
              (if num
                (if (contains? seen num)
                  true
                  (go (first nums) (rest nums) (conj seen num)))
                false))]
    (go (first nums) (rest nums) #{})))

(defn contains-duplicate-2 [nums]
  (not= (count nums) (count (into #{} nums))))

(defn contains-duplicate-3 [nums]
  (loop [nums nums seen #{}]
    (if (empty? nums)
      false
      (let [num (first nums)]
        (if (contains? seen num)
          true
          (recur (rest nums) (conj seen num)))))))

(defmacro test-function [f]
  (let [function-name (gensym (name f))]
    `(deftest
       ~function-name
       (testing "example 1 "
         (is (= true (~f '(1 2 3 1)))))
       (testing "example 2 "
         (is (= false (~f '(1 2 3 4)))))
       (testing "example 3 "
         (is (= true (~f '(1 1 1 3 3 4 3 2 4 2))))))))

(test-function contains-duplicate)
(test-function contains-duplicate-2)
(test-function contains-duplicate-3)


(ns core
  (:require [clojure.test :refer [deftest testing is]]))

; You are a professional robber planning to rob houses along a street.
; Each house has a certain amount of money stashed, the only constraint stopping you from robbing each of them is that adjacent houses have security systems connected and it will automatically contact the police if two adjacent houses were broken into on the same night.
;
; Given an integer array nums representing the amount of money of each house,
; return the maximum amount of money you can rob tonight without alerting the police.
;
; Example 1:
;
; Input: nums = [1,2,3,1]
; Output: 4
; Explanation: Rob house 1 (money = 1) and then rob house 3 (money = 3).
; Total amount you can rob = 1 + 3 = 4.
; Example 2:
;
; Input: nums = [2,7,9,3,1]
; Output: 12
; Explanation: Rob house 1 (money = 2), rob house 3 (money = 9) and rob house 5 (money = 1).
; Total amount you can rob = 2 + 9 + 1 = 12.
;
; Constraints:
;
; 1 <= nums.length <= 100
; 0 <= nums[i] <= 400
;
; time O(n)
; space O(n)
;

;; time O(n)
;; space O(n)
(defn rob [houses]
  (letfn [(go [dp house-number]
              (if (< house-number 0)
                (get dp 0)
                (let [money (get houses house-number)
                      max-money (max (get dp (+ house-number 1) 0) (+ money (get dp (+ house-number 2) 0)))]
                  (go (assoc dp house-number max-money) (dec house-number)))))]
    (go (vec (repeat (count houses) 0)) (- (count houses) 1))))

;; time O(n)
;; space O(1)
(defn rob-2 [houses]
  (letfn [(go [rob-1 rob-2 house-number]
              (if (< house-number 0)
                rob-1
                (let [money (get houses house-number)
                      max-money (max rob-1 (+ money rob-2))]
                  (go max-money rob-1 (dec house-number)))))]
    (go 0 0 (- (count houses) 1))))

(defmacro test-function [f]
  (let [function-name (gensym (name f))]
    `(deftest
       ~function-name
       (testing " example 1 "
         (is (= 4 (~f [1 2 3 1]))))
       (testing "example 2"
         (is (= 12 (~f [2 7 9 3 1]))))
       (testing "example 3"
         (is (= 1 (~f [1]))))
       (testing "example 4"
         (is (= 4 (~f [2 1 1 2])))))))

(test-function rob)
(test-function rob-2)

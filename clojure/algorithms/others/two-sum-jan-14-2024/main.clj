(ns core
  (:require [clojure.test :refer [deftest testing is]]
            [clojure.string]))

;; Given an array of integers nums and an integer target, 
;; return indices of the two numbers such that they add up to target.
;; 
;; You may assume that each input would have exactly one solution, and you may not use the same element twice.
;; 
;; You can return the answer in any order.
;; 
;; Example 1:
;; 
;; Input: nums = [2,7,11,15], target = 9
;; Output: [0,1]
;; Explanation: Because nums[0] + nums[1] == 9, we return [0, 1].
;; Example 2:
;; 
;; Input: nums = [3,2,4], target = 6
;; Output: [1,2]
;; Example 3:
;; 
;; Input: nums = [3,3], target = 6
;; Output: [0,1]
;;  
;; Constraints:
;; 
;; 2 <= nums.length <= 104
;; -109 <= nums[i] <= 109
;; -109 <= target <= 109
;; Only one valid answer exists.
;;  
;; Follow-up: Can you come up with an algorithm that is less than O(n2) time complexity?

;; time O(n)
;; sçace O(n)
(defn two-sum [nums target]
  (letfn [(go [cache nums]
              (let [[index num] (first nums)
                    diff (abs (- target num))
                    other-value-index (get cache diff)]
                (if other-value-index
                  [other-value-index index]
                  (recur (assoc cache num index) (rest nums)))))]
    (go {} (map-indexed vector nums))))

(defmacro test-function [f]
  (let [function-name (gensym (name f))]
    `(deftest
       ~function-name
       (testing "example 1"
         (is (= [0 1] (~f [2 7 11 15] 9))))
       (testing "example 2"
         (is (= [1 2] (~f [3 2 4] 6))))
       (testing "example 3"
         (is (= [0 1] (~f [3 3] 6)))))))

(test-function two-sum)


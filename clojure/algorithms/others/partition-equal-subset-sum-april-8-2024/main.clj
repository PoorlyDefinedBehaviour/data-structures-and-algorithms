(ns core
  (:require [clojure.test :refer [deftest is testing]]))

(comment
  "Given an integer array nums, return true if
you can partition the array into two subsets such that the sum of the elements in both subsets is equal or false otherwise.
   
Example 1:

Input: nums = [1,5,11,5]
Output: true
Explanation: The array can be partitioned as [1, 5, 5] and [11].
Example 2:

Input: nums = [1,2,3,5]
Output: false
Explanation: The array cannot be partitioned into equal sum subsets.
 

Constraints:

1 <= nums.length <= 200
1 <= nums[i] <= 100")

;; time O(n * sum(nums))
;; space O(sum(nums))
(defn can-partition [nums]
  ;; O(n)
  (let [sum (reduce + nums)
        half (/ sum 2)]
    (if (odd? sum)
      false
      ;; O(n)
      (let [result (reduce (fn [seen num]
                             ;; O(sum(nums))
                             (let [new-seen (reduce (fn [new-seen other-num] (let [total (+ num other-num)]
                                                                               (if (<= total half)
                                                                                 (conj new-seen total)
                                                                                 new-seen)))
                                                    seen
                                                    seen)]
                               (if (contains? new-seen half)
                                 (reduced true)
                                 new-seen)))
                           #{0}
                           nums)]
        (= result true)))))

(deftest can-partition-test
  (testing "example 1"
    (is (= true (can-partition '(1 5 11 5))))
    (is (= false (can-partition '(1 2 3 5))))
    (is (= true (can-partition '(1 1))))))

(clojure.test/run-all-tests)
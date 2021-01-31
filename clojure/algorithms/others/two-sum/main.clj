(ns kata.core)
(use 'clojure.test)

(comment "
Given an array of integers nums and an integer target, return indices of the two numbers such that they add up to target.

You may assume that each input would have exactly one solution, and you may not use the same element twice.

You can return the answer in any order.

Example 1:

Input: nums = [2,7,11,15], target = 9
Output: [0,1]
Output: Because nums[0] + nums[1] == 9, we return [0, 1].

Example 2:

Input: nums = [3,2,4], target = 6
Output: [1,2]

Example 3:

Input: nums = [3,3], target = 6
Output: [0,1]
")

;; time O(n)
;; space O(n)
(defn two-sum [numbers target]
  (let [pair (reduce-kv (fn [differences index number]
                          (let [difference (Math/abs (- target number))
                                other-index (get differences number)]
                            (if (nil? other-index)
                              (assoc differences difference index)
                              (reduced (list other-index index)))))
                        {}
                        numbers)]
    (if (list? pair)
      pair
      nil)))

(deftest two-sum-tests
  (is (= '(0 1) (two-sum [2 7 11 15] 9)))
  (is (= '(1 2) (two-sum [3 2 4] 6)))
  (is (= '(0 1) (two-sum [3 3] 6))))

(run-tests)


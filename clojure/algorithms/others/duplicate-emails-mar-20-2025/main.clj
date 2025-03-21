(ns mini.playground
  (:require
   [clojure.test :refer [deftest is testing]]))

;; +-------------+---------+
;; | Column Name | Type    |
;; +-------------+---------+
;; | id          | int     |
;; | email       | varchar |
;; +-------------+---------+
;; id is the primary key (column with unique values) for this table.
;; Each row of this table contains an email. The emails will not contain uppercase letters.

;; Write a solution to report all the duplicate emails. Note that it's guaranteed that the email field is not NULL.

;; Return the result table in any order.

;; The result format is in the following example.

;; Example 1:

;; Input: 
;; Person table:
;; +----+---------+
;; | id | email   |
;; +----+---------+
;; | 1  | a@b.com |
;; | 2  | c@d.com |
;; | 3  | a@b.com |
;; +----+---------+
;; Output: 
;; +---------+
;; | Email   |
;; +---------+
;; | a@b.com |
;; +---------+
;; Explanation: a@b.com is repeated two times.


;; time O(n)
;; space O(n)
(defn duplicated-emails [rows]
  (let [freq (frequencies (map :email rows))
        duplicated (filter (fn [[_ frequency]] (> frequency 1)) freq)]
    (map first duplicated)))

(deftest duplicated-emails-test
  (testing "basic"
    (is (= ["a@b.com"] (duplicated-emails [{:id 1 :email "a@b.com"}
                                           {:id 2 :email "c@d.com"}
                                           {:id 3 :email "a@b.com"}])))))



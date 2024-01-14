(ns core
  (:require [clojure.test :refer [deftest testing is]]
            [clojure.string]))

;; Given two strings s and t, return true if t is an anagram of s, and false otherwise.
;; 
;; An Anagram is a word or phrase formed by rearranging the letters of a different word or phrase, 
;; typically using all the original letters exactly once.
;; 
;; Example 1:
;; 
;; Input: s = "anagram", t = "nagaram"
;; Output: true
;; Example 2:
;; 
;; Input: s = "rat", t = "car"
;; Output: false
;;  
;; Constraints:
;; 
;; 1 <= s.length, t.length <= 5 * 104
;; s and t consist of lowercase English letters.

;; time O(n log n)
;; space O(n)
(defn is-anagram [s t]
  (if (not= (count s) (count t))
    false
    (= (sort s) (sort t))))

;; time O(n)
;; space O(n)
(defn is-anagram-2 [s t]
  (if (not= (count s) (count t))
    false
    (= (frequencies s) (frequencies t))))

;; time O(n)
;; space O(1)
(defn is-anagram-3 [s t]
  (if (not= (count s) (count t))
    false
    (letfn [(frequency [frequency-vec s]
              (if (empty? s)
                frequency-vec
                (let [letter (first s)
                      letter-ascii-value (- (int letter) (int \a))
                      current-value (get frequency-vec letter)
                      new-value (if current-value (inc current-value) 1)
                      new-frequency-vec (assoc frequency-vec letter-ascii-value new-value)]
                  (recur new-frequency-vec (rest s)))))]
      (= (frequency (vec (repeat 26 0)) s) (frequency (vec (repeat 26 0)) t)))))

(defmacro test-function [f]
  (let [function-name (gensym (name f))]
    `(deftest
       ~function-name
       (testing "example 1 "
         (is (= true (~f "anagram" "nagaram"))))
       (testing "example 2 "
         (is (= false (~f "rat" "car")))))))

(test-function is-anagram)
(test-function is-anagram-2)
(test-function is-anagram-3)

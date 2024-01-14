(ns core
  (:require [clojure.test :refer [deftest testing is]]
            [clojure.string]))

;; Given an array of strings strs, group the anagrams together. You can return the answer in any order.
;; 
;; An Anagram is a word or phrase formed by rearranging the letters of a different word or phrase, 
;; typically using all the original letters exactly once.
;; 
;; Example 1:
;; 
;; Input: strs = ["eat","tea","tan","ate","nat","bat"]
;; Output: [["bat"],["nat","tan"],["ate","eat","tea"]]
;; Example 2:
;; 
;; Input: strs = [""]
;; Output: [[""]]
;; Example 3:
;; 
;; Input: strs = ["a"]
;; Output: [["a"]]
;;  
;; Constraints:
;; 
;; 1 <= strs.length <= 104
;; 0 <= strs[i].length <= 100
;; strs[i] consists of lowercase English letters.


(defn frequency [s]
  (letfn [(go [frequency-vec s]
              (if (empty? s)
                frequency-vec
                (let [index (- (int (first s)) (int \a))
                      count (get frequency-vec index)
                      new-frequency-vec (assoc frequency-vec index (inc count))]
                  (recur new-frequency-vec (rest s)))))]
    (go (vec (repeat 26 0)) s)))

;; time O(n * m)
;; space O(n * m)
(defn group-anagrams [strs]
  (vals (reduce (fn [groups str]
                  (let [freq (frequency str)
                        group (get groups freq [])]
                    (assoc groups freq (conj group str)))) {} strs)))

(defmacro test-function [f]
  (let [function-name (gensym (name f))]
    `(deftest
       ~function-name
       (testing "example 1"
         (is (= '(["eat" "tea" "ate"] ["tan" "nat"] ["bat"]) (~f ["eat","tea","tan","ate","nat","bat"]))))
       (testing "example 2"
         (is (= '([""]) (~f [""])))
         (testing "example 3"
           (is (= '(["a"]) (~f ["a"]))))))))

(test-function group-anagrams)

(ns kata.core)
(use 'clojure.test)
(require 'clojure.string)

;; Given a string containing just the characters '(', ')', '{', '}', '[' and ']', determine if the input string is valid.
;; An input string is valid if :
;; Open brackets must be closed by the same type of brackets.Open brackets must be closed in the correct order.
;; Note that an empty string is also considered valid.
;; Examples:
;; Input : "()" Output : true
;; Input : "()[]{}" Output : true
;; Input : "(]" Output : false

(defn is-opening [character]
  (case character
    "(" true
    "[" true
    "{" true
    false))

(defn is-closing [character]
  (case character
    ")" true
    "]" true
    "}" true
    false))

(defn matches [character-a character-b]
  (case character-b
    ")" (= character-a "(")
    "]" (= character-a "[")
    "}" (= character-a "{")
    false))

(defn matches-last-character [characters character]
  (if (empty? characters) false
      (let [last-character (last characters)]
        (matches last-character character))))

(defn consumed-all-characters [result]
  (if (vector? result)
    (empty?  result)
    result))

(defn is-string-balanced [s]
  (let [result (reduce (fn [characters character]
                         (if (is-opening character)
                           (conj characters character)
                           (if (not (matches-last-character characters character))
                             (reduced false)
                             (vec (drop-last 1 characters)))))
                       []
                       (clojure.string/split s #""))]
    (consumed-all-characters result)))

(deftest is-string-balanced-tests
  (is (= true (is-string-balanced "()")))
  (is (= true (is-string-balanced "()[]{}")))
  (is (= false (is-string-balanced "(]")))
  (is (= false (is-string-balanced "()()()(()))"))))

(run-tests)
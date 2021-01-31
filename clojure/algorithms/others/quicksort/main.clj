(ns kata.core)
(use 'clojure.test)

(defn quicksort-1 [array]
  (if (empty? array)
    array
    (let [pivot (first array)
          tail (rest array)]
      (concat
       (quicksort-1 (filter #(< % pivot) tail))
       (list pivot)
       (quicksort-1 (filter #(> % pivot) tail))))))

(defn quicksort-2 [array]
  (if (empty? array)
    array
    (let [pivot (first array)
          tail (rest array)
          [smaller greater] (split-with #(< % pivot) tail)]
      (concat
       (quicksort-2 smaller)
       (list pivot)
       (quicksort-2 greater)))))

(deftest quicksort-1-tests
  (is (= '(1 2 3) (quicksort-1 '(3 2 1))))
  (is (= '(-1 0) (quicksort-1 '(0 -1))))
  (is (= '(-20 -10) (quicksort-1 '(-10 -20))))
  (is (= '() (quicksort-1 '()))))

(deftest quicksort-2-tests
  (is (= '(1 2 3) (quicksort-2 '(3 2 1))))
  (is (= '(-1 0) (quicksort-2 '(0 -1))))
  (is (= '(-20 -10) (quicksort-2 '(-10 -20))))
  (is (= '() (quicksort-2 '()))))

(run-tests)

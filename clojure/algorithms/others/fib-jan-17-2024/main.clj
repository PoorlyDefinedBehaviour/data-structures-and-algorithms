(defn fib [n]
  (cond
    (= 0 n) 0
    (= 1 n) 1
    :else (+ (fib (- n 1)) (fib (- n 2)))))

(defn fib-2 [n]
  (if (= n 0)
    0
    (let [previous-previous (atom 0)
          previous (atom 1)]
      (dotimes [_ (- n 1)]
        (let [current (+ @previous-previous @previous)]
          (println "current" current)
          (reset! previous-previous @previous)
          (reset! previous current)))
      @previous)))


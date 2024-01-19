
;; I'm aware only the last 2 numbers are needed.
;; 1, 1, 2, 3, 5, 8, 13 and 21
(defn fib [n]
  (letfn [(go [i cache] 
              (let [nth-fib-number (+ (get cache (- i 1)) (get cache (- i 2)))]
                (if (>= i n)
                  nth-fib-number
                  (recur (inc i) (assoc cache i nth-fib-number))))
              )]
    (go 2 (-> (vec (repeat n 0))
              (assoc 0 0)
              (assoc 1 1)))))


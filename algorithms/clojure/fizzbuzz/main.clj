(defn fizzbuzz [x]
  (let [v [(= 0 (mod 3 x)) (= 0 (mod 5 x))]]
    ({[true, true] "fizzbuzz"
      [true, false] "fizz"
      [false, true] "buzz"
      [false, false] x}
     v)))


(doseq [x (map fizzbuzz (range 1 16))] (println x))
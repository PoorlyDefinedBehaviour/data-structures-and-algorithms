(ns mini.playground)

(defn count-days-without-meetings [days meetings]
  (- days (count (into #{} (flatten (map (fn [[start end]] (range start (inc end))) meetings))))))

(defn count-days-without-meetings-2 [days meetings]
  (let [sorted-meetings (sort-by (fn [[start _end]] start) meetings)]
    (loop [previous-end 0
           days days
           meetings sorted-meetings]
      (if (empty? meetings)
        days
        (let [[meeting-start meeting-end] (first meetings)
              start (max meeting-start (inc previous-end))
              length (+ 1 (- meeting-end start))
              days (- days (max length 0))
              new-previous-end (max previous-end meeting-end)]
          (recur new-previous-end days (rest meetings)))))))

(comment
  (count-days-without-meetings-2 10 [[5 7] [1 3] [9 10]])

  (count-days-without-meetings 10 [[5 7] [1 3] [9 10]])
  ;
  )
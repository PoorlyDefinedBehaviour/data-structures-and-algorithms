(ns mini.playground)

(defn apply2 [f & args]
  (assert (seq args) "at least one argument is required")
  (let [result (f (first args))
        args (rest args)]
    (loop [result result
           args args]
      (if (empty? args)
        result
        (recur (f result (first args)) (rest args))))))


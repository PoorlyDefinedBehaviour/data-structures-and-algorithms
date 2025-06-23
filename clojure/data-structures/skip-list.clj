(ns main
  (:require
   [clojure.test :refer [run-all-tests]]
   [clojure.test.check :as tc]
   [clojure.test.check.generators :as gen]
   [clojure.test.check.properties :as prop]))

(defrecord Config [max-level])

(defn new-skip-list [cfg]
  {:cfg cfg
   :random (java.util.Random. (System/currentTimeMillis))
   :levels (vec (repeat (:max-level cfg) []))})

(defn coin-flip [skip-list]
  (.nextBoolean (:random skip-list)))

;; NOTE: could binary search
(defn find-index [pred xs]
  (loop [xs xs i 0]
    (cond
      (empty? xs) nil
      (pred (first xs)) i
      :else (recur (rest xs) (inc i)))))

(defn descend [skip-list target]
  (loop [level 0
         path []]
    (if (>= level (-> skip-list :cfg :max-level))
      path
      (let [entries (nth (:levels skip-list) level)
            index (find-index #(>= %1 target) entries)]
        (recur (inc level) (conj path (if (nil? index)
                                        nil
                                        (dec index))))))))
(defn search [pred skip-list]
  (loop [level 0]
    (if (>= level (-> skip-list :cfg :max-level))
      nil
      (let [entries (nth (:levels skip-list) level)
            index (find-index pred entries)]
        (if (nil? index)
          (recur (inc level))
          {:level level
           :index index})))))

(defn insert [skip-list value]
  (let [path (descend skip-list value)
        max-level (dec (-> skip-list :cfg :max-level))]
    (loop [levels (:levels skip-list)
           level max-level]
      (if (or (= level max-level) (and (>= level 0) (coin-flip skip-list)))
        (let [node (nth path level)
              entries (levels level)
              updated-entries (if (nil? node)
                                (conj entries value)
                                (let [pieces (split-at (inc node) entries)]
                                  (vec (concat (first pieces) [value] (second pieces)))))]
          (recur (assoc levels level updated-entries) (dec level)))
        (assoc skip-list :levels levels)))))

(let [skip-list (new-skip-list (map->Config {:max-level 3}))]
  ;; (descend (assoc skip-list :levels [[2], [1] ,[1, 2, 3]]) 4)
  (let [skip-list (insert skip-list 3)
        _ (println " insert 3 " skip-list)
        skip-list (insert skip-list 1)
        _ (println " insert 1 " skip-list)
        skip-list (insert skip-list 2)
        _ (println " insert 2 " skip-list)]
    (search #(do
               (println "in search" %1)
               (= %1 2)) skip-list)))

(def skip-list-values-are-sorted-at-all-levels-prop
  (prop/for-all [v (gen/vector gen/int)]
                (println v)))

(tc/quick-check 100 skip-list-values-are-sorted-at-all-levels-prop)

(run-all-tests)
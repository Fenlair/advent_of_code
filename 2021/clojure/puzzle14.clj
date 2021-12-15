(ns advent-of-code
  (:require [clojure.string :as str]))

(def example ["NNCB", "", "CH -> B", "HH -> N", "CB -> H", "NH -> C", "HB -> C", "HC -> B", "HN -> C", "NN -> C", "BH -> H", "NC -> B", "NB -> B", "BN -> B", "BB -> N", "BC -> B", "CC -> N", "CN -> C"])
(def real (str/split-lines (slurp "2021/inputs/day14.txt")))

(let [[template _ & tmp] example]
  (def template template)
  (def lookup (into {} (map #(str/split % #" -> ") tmp))))

(defn step
  ([l r] (step l r 40))
  ([l r depth]
   (if (= depth 0)
     {}
     (let [m (get lookup (str l r))]
       (merge-with + {m 1} (step l m (dec depth)) (step m r (dec depth)))))))
(def step-memo (memoize step))

(defn freqs [s]
  (let [f (frequencies s)]
    (into {} (map #(vector (str %1) %2) (keys f) (vals f)))))

(defn solve []
  (->> (reduce #(merge-with + %1 %2)
               (freqs template)
               (map step-memo template (drop 1 template)))
       (vals)
       (apply (juxt max min))
       (apply -)))
(println "Puzzle2:" (solve))

(ns advent-of-code
  (:require [clojure.string :as str]))
(declare parse-input neighbours low? low-points sizeof-basin)

(def input-data (-> (slurp "09/input.txt") (str/split-lines)))
(def example-data ["2199943210" "3987894921" "9856789892" "8767896789" "9899965678"])
(def grid (parse-input input-data))

(time (println "puzzle1:" (->> (low-points grid)
                               (map #(inc (second %)))
                               (reduce +))))

(time (println "puzzle2:" (->> (low-points grid)
                               (map #(sizeof-basin % grid))
                               sort
                               reverse
                               (take 3)
                               (reduce *))))

(defn parse-input [lines]
  (into {} (for [[m line] (map-indexed vector lines)
                 [n h]    (map-indexed vector line)]
             [[m n] (Integer/parseInt (str h))])))

(defn neighbours [[m n] grid]
  (->> [[(inc m) n] [(dec m) n] [m (inc n)] [m (dec n)]]
       (filter #(grid %))))

(defn low? [[p h] grid]
  (every? #(< h (grid %)) (neighbours p grid)))

(defn low-points [grid]
  (filter #(low? % grid) grid))

(defn helper-basin [p grid visited]
  (cond
    (= (grid p) 9) 0
    (@visited p)   0
    :default (do (swap! visited conj p)
                 (reduce + 1 (map #(helper-basin % grid visited) (neighbours p grid))))))

(defn sizeof-basin [[p v] grid]
  (let [visited (atom #{})]
    (helper-basin p grid visited)))

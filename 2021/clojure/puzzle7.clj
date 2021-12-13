(ns puzzle07
  (:require [clojure.string :as str]))
(def example-data "16,1,2,0,4,2,7,1,2,14")

(defn parse-data [data]
  (->> (str/split data #",")
       (map #(Integer/parseInt %))))

(defn map-abs-diff [a seq]
  (map #(Math/abs (- a %)) seq))

(defn crab-costs [positions pos]
  (->> (map-abs-diff pos positions)
       (reduce +)))

(defn best-position [positions crab-fn]
  (->> (range (count positions))
       (map #(crab-fn positions %))
       sort
       first))

(defn revised-crab-costs [positions pos]
  (->> (map-abs-diff pos positions)
       (map #(range 1 (inc %)))
       (map #(reduce + %))
       (reduce +)))

(def positions (parse-data example-data))
(println "Puzzle1" (best-position positions crab-costs))
(println "Puzzle2" (best-position positions revised-crab-costs))

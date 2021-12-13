#!/usr/bin/env bb
(ns advent-of-code)

(def example-data [199 200 208 210 200 207 240 269 260 263])
(def data (->> (slurp "./workspace/clojure/advent-of-code/01/input.txt")
               (clojure.string/split-lines)
               (map #(Integer/parseInt %))))

(defn diff [coll]
  (map - (rest coll) coll))

(defn how-many-positive [coll]
  (->> (map pos? coll)
       (filter identity)
       count))

(defn how-many-increased [coll]
  (->> (diff coll)
       how-many-positive))

(defn sum-sublists [coll]
  (map #(apply + %) coll))

(defn sliding-window-sum3 [coll]
  (-> (partition 3 1 coll)
      sum-sublists))

(print (how-many-increased data))

(print (-> (sliding-window-sum3 data)
           how-many-increased))

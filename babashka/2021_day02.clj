#!/usr/bin/env bb
(ns advent-of-code
  (:require [clojure.string :as str]))

(defn parse-int [number-string] (try (Integer/parseInt number-string) (catch Exception e nil)))
(defn parse-2nd-to-int [[dir value]] [dir (parse-int value)])

(def data (->> (slurp "../inputs/2021_day02.txt")
               str/split-lines
               (map #(str/split % #" "))
               (map parse-2nd-to-int)))

(def example-data [["forward" 5] ["down" 5] ["forward" 8] ["up" 3] ["down" 8] ["forward" 2]])

(defn next-step [inst hor depth]
  (let [[direction value] inst]
    (case direction
    "forward" [(+ hor value) depth]
    "down"    [hor (+ depth value)]
    "up"      [hor (- depth value)])))

(defn execute-steps [start-data]
  (loop [data start-data
         [hor depth] [0 0]]
    (if (> (count data) 1)
      (recur (rest data) (next-step (first data) hor depth))
      (next-step (first data) hor depth))))

(print (apply * (execute-steps data)))

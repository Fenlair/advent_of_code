(ns advent-of-code
  (:require [clojure.string :as str]))

(def example ["00100" "11110" "10110" "10111" "10101" "01111" "00111" "11100" "10000" "11001" "00010" "01010"])
(def real (-> (slurp "../inputs/2021_day03.txt") (str/split-lines)))

(defn parse [input]
  (for [n input] (map #(Character/digit % 10) n)))

(defn most-common-bit [data pos]
  (let [sum (reduce + (map #(nth % pos) data))]
    (if (>= sum (/ (count data) 2)) 1 0)))

(defn most-common-binary [data]
  (vec (map #(most-common-bit data %) (range (count (first data))))))

(defn complement-binary [binary]
  (vec (map #(- 1 %) binary)))

(defn bin-to-dec [binary]
  (reduce + (for [[ind bit] (map-indexed vector (reverse binary))]
              (* bit (Math/pow 2 ind)))))

(def data (parse real))
(let [gamma (most-common-binary data)
      epsilon (complement-binary gamma)]
  (println "Puzzle1:" (* (bin-to-dec gamma) (bin-to-dec epsilon))))

(defn filter-data [remaining-data ffn ind]
  (if (<= (count remaining-data) 1)
    (vec (flatten remaining-data))
    (let [filter-bit (ffn remaining-data ind)]
      (recur (filter #(= (nth % ind) filter-bit) remaining-data) ffn (inc ind)))))

(let [oxygen (filter-data data most-common-bit 0)
      co2 (filter-data data (comp #(- 1 %) most-common-bit) 0)]
  (println "Puzzle2:" (* (bin-to-dec oxygen) (bin-to-dec co2))))

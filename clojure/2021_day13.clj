(ns advent-of-code
  (:require [clojure.string :as str]
            [clojure.set :as set]))

(let [[dotdata folddata] (str/split (slurp "../inputs/2021_day13.txt") #"\n\n")]
  (def dots (->> dotdata
                 str/split-lines
                 (map #(str/split % #","))
                 (map #(vec (map #(Integer/parseInt %) %)))
                 (into #{})))
  (def folds (->> (str/split folddata #"\s")
                  (drop 2)
                  (take-nth 3)
                  (map #(str/split % #"="))
                  (map #(assoc %1 1 (Integer/parseInt (second %1)))))))

(defn fold [[dir value] dots]
  (let [fold?        (fn [v]  (> (if (= dir "x") (first v) (second v)) value))
        apply-x-or-y (fn [fun [x y]] (if (= dir "x") [(fun x) y] [x (fun y)]))
        to-be-folded (filter fold? dots)
        new-dots (->> to-be-folded
                      (map #(apply-x-or-y #(- (* 2 value) %) %))
                      (into #{}))]
    (set/union (set/difference dots to-be-folded) new-dots)))

(defn do-folds [folds dots]
  (if (empty? folds)
    dots
    (recur (rest folds) (fold (first folds) dots))))

(defn print-dots [dots]
  (let [xmax (apply max (map first dots))
        ymax (apply max (map second dots))]
    (doseq [y (range (inc ymax))]
      (doseq [x (range (inc xmax))]
        (print (if (get dots [x y]) "# " ". ")))
      (println))))

(println "Puzzle1:" (count (fold (first folds) dots)))
(time (print-dots (do-folds folds dots)))

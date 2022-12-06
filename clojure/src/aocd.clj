(ns aocd
  (:require [clj-http.client :as client]
            [clojure.java.io :as io]
            clojure.string))

(def aocd-dir "/home/pascal/.config/aocd/")
(def aocd-user-dir (str aocd-dir "reddit.RealFenlair.1870387/"))
(def token (clojure.string/trim-newline (slurp (str aocd-dir "token"))))
;; (def resp (client/get "https://adventofcode.com/2022/day/1/input"
;;                       {:headers {"Cookie" (str "session=" token)}}))

(defn get-data [day year]
  (let [cache-file (str aocd-user-dir year "_" day "_input.txt")]
    (prn cache-file)
    (if (.exists (io/file cache-file))
      (slurp cache-file)
      (let [resp (client/get
                  (str "https://adventofcode.com/" year "/day/" day "/input")
                  {:headers {"Cookie" (str "session=" token)}})]
        (if (not= (:status resp) 200)
          (throw (Exception. "Bad status."))
          (:body resp))))))

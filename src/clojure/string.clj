(ns clojure.string)

"TODO : some special syntax required because of missing require"

(def split-lines
  (fn [s]
    (split s #"\r?\n")))
"clojure.string"

"TODO : some special syntax required because of missing require"

(def clojure.string/split-lines
  (fn [s]
    (clojure.string/split s #"\r?\n")))
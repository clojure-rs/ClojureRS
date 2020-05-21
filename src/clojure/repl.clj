(ns clojure.repl)

(defmacro doc [name]
  (print-doc name))
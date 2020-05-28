;; the content here should include the minimum implementation for interpreting clojure.core library code
;; currently, most things existing here is for testing the implementation

(ns clojure.core)

(def *flush-on-newline* true)
(def *print-readably* true)

"Temporary work around for exceptions, should be special form"
(defmacro throw
  [exception-form]
  (println (str (first exception-form)) ": " (second exception-form)))

(defmacro when [test & body]
  (list 'if test (cons 'do body)))

(def list (fn [& ls] ls))

(defmacro defn [name args & body]
  (list (quote def) name 
        (list (quote fn) args 
              (concat (list (quote do)) body))))

(defn apply [f args]
  (lexical-eval (concat (list f) args)))

(defn newline
  []
  (system-newline))

(defn flush
  []
  (flush-stdout))

(defn pr [& more]
  (print-string (apply str more)))

(defn prn [& more]
  (apply pr more)
  (newline)
  (when *flush-on-newline*
    (flush)
    nil))

(defn print [& more]
  (apply pr more))

(defn println [& more]
  (apply prn more))

(defn inc [x]
  (+ x 1))

(defn dec [x]
  (- x 1))

(defmacro time [expr]
  (list (quote let) [(quote start) (quote (System/nanoTime)) (quote ret) expr]
        (quote (do
        (println (str "Elapsed time: " (_slash_ (- (System/nanoTime) start) 1000000.0) " msecs"))
        ret))))

(defn slurp [f & opts]
  (rust-slurp f opts))

"basic operations on collections"

(defn rest [x]
  (more x))

(defn next [x]
  (let [result (rest x)]
    (if (= '() result)
      nil
      result)))

(defn ffirst [x]
  (first (first x)))

(defmacro doc [name]
  (print-doc name))

(defmacro var [name]
  (var-special-form name))

"proof of concept: cond as expressed (almost) in Clojure"
(defmacro cond
  [& clauses]
  (when clauses
    (list 'if (first clauses)
          (if (next clauses)
            (second clauses)
            (throw (IllegalArgumentException
                     "cond requires an even number of forms")))
          (cons 'clojure.core/cond (next (next clauses))))))

;; Define 'Classes' with aliases so that we have types

(def rust.std.i32 (deftype-rs "rust.std.i32"))
(def java.lang.Long rust.std.i32)
(def java.lang.Integer rust.std.i32)
(def Integer rust.std.i32)
(def Long rust.std.i32)

(def rust.std.bool (deftype-rs "rust.std.bool"))
(def java.lang.Boolean rust.std.bool)
(def Boolean rust.std.bool)

(def rust.std.f64 (deftype-rs "rust.std.f64"))
(def java.lang.Double rust.std.f64)
(def Double rust.std.f64)

(def rust.std.string.String (deftype-rs "rust.std.string.String"))
(def java.lang.String rust.std.string.String)
(def String rust.std.string.String)
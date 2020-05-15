"strings work as temporary comments"

(def *flush-on-newline* true)
(def *print-readably* true)

"TODO: #Condition[Execution Error: clojure.lang.Nil cannot be cast to clojure.lang.IFn] when succesful"
"Bug in do"

(defmacro when [test & body]
  (list 'if test (list 'do body) nil))

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

"TODO: use when"
(defn prn [& more]
  (apply pr more)
  (newline)
  (if *flush-on-newline*
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
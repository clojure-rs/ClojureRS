(def *flush-on-newline* true)
(def *print-readably* true)

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
  (flush))

(defn print [& more]
  (apply pr more))

(defn println [& more]
  (apply prn more))

(defn inc [x]
  (+ x 1))

(defn dec [x]
  (- x 1))

(defmacro time [expr]
  (list (quote let) [(quote start) (quote (System_nanotime)) (quote ret) expr]
        (quote (do
        (println (str "Elapsed time: " (_slash_ (- (System_nanoTime) start) 1000000.0) " msecs"))
        ret))))

(defn slurp [f & opts]
  (rust-slurp f opts))

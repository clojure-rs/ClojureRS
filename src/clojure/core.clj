(def list (fn [& ls] ls))

(defmacro defn [name args & body]
  (list (quote def) name 
        (list (quote fn) args 
              (concat (list (quote do)) body))))

(defn apply [f args]
  (lexical-eval (concat (list f) args)))

(defn println [& more]
  (print-string (apply str more)))

(defn inc [x]
  (+ x 1))

(defn dec [x]
  (- x 1))

(defmacro time [expr]
  (list (quote let) [(quote start) (quote (System_nanotime)) (quote ret) expr]
        (quote (do
        (println (str "Elapsed time: " (_slash_ (- (System_nanotime) start) 1000000.0) " msecs"))
        ret))))

(defn slurp [f & opts]
  (rust-slurp f opts))

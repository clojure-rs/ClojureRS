;(ns clojure.interop)

;; Interop for reading clojure.core
;; namespaced symbols should be considered private

;;;
; types
;;

;; We define 'Classes' with aliases so that we have types

(def rust.std.i32 (clojure.interop/deftype-rs "rust.std.i32"))
(def java.lang.Long rust.std.i32)
(def java.lang.Integer rust.std.i32)
(def Integer rust.std.i32)
(def Long rust.std.i32)

(def rust.std.bool (clojure.interop/deftype-rs "rust.std.bool"))
(def java.lang.Boolean rust.std.bool)
(def Boolean rust.std.bool)

(def rust.std.f64 (clojure.interop/deftype-rs "rust.std.f64"))
(def java.lang.Double rust.std.f64)
(def Double rust.std.f64)

(def rust.std.string.String (clojure.interop/deftype-rs "rust.std.string.String"))
(def java.lang.String rust.std.string.String)
(def String rust.std.string.String)

(def clojure.lang.PersistentList (clojure.interop/deftype-rs "clojure.lang.PersistentList"))

(defn clojure.interop/list?
  [x]
  (= (type x) clojure.lang.PersistentList))

(defmacro clojure.interop/->interop-fn
  [interop-ns interop-symbol]
  (list 'eval (list 'clojure.interop/symbol (str interop-ns) (str interop-symbol))))

;;;
; interpreting dot forms
;;

(defmacro clojure.interop/dot-symbol
  [interop-ns form]
  (list
   'do
   (list 'eval (list 'clojure.interop/->interop-fn interop-ns form))))

(defmacro .
  [interop-ns & form]
  (cond
    (clojure.interop/lt 1 (clojure.interop/count (first form))) ; ((println 1 2 3))
    (list 'apply (cons 'clojure.interop/dot-symbol [interop-ns (ffirst form)]) (list 'rest (first form)))
    (= 1 (clojure.interop/count (first form))) ; (. clojure.core (println))
    (list 'eval (list (cons 'clojure.interop/dot-symbol [interop-ns (ffirst form)])))
    :else ; returns the symbol, i.e. clojure.core/println
    (cons 'clojure.interop/dot-symbol [interop-ns (first form)])))

; hooks for dot forms

;; line 20 clojure.core, this actually calls the constructor of the Primordial class
(def clojure.lang.PersistentList/creator clojure.core/list)
;; line 29
(def clojure.lang.RT/cons clojure.core/cons)

(def
  ^{:arglists '([x seq])
    :doc "Returns a new seq where x is the first element and seq is
    the rest."
    :added "1.0"
    :static true}

  cons2 (fn* ^:static cons [x seq] (. clojure.lang.RT (cons x seq))))

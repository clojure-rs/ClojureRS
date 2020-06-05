(ns clojure.interop)

;; Interop for reading clojure.core

;;;
;    Types
;;

;; We define 'Classes' with aliases so that we have types
;; the first one in a group is the (only or) clojureRS type name
;; and the type names following the first are aliases for interop
;; (when available)

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

(def clojure.lang.Symbol (clojure.interop/deftype-rs "clojure.lang.Symbol"))

(def clojure.lang.Class (clojure.interop/deftype-rs "clojure.lang.Class"))

(def clojure.lang.Keyword (clojure.interop/deftype-rs "clojure.lang.Keyword"))

(def clojure.lang.IFn (clojure.interop/deftype-rs "clojure.lang.IFn"))
(def clojure.lang.Function clojure.lang.IFn)

(def clojure.lang.Condition (clojure.interop/deftype-rs "clojure.lang.Condition"))

(def clojure.lang.PersistentList (clojure.interop/deftype-rs "clojure.lang.PersistentList"))

(def clojure.lang.PersistentVector (clojure.interop/deftype-rs "clojure.lang.PersistentVector"))

(def clojure.lang.PersistentArrayMap (clojure.interop/deftype-rs "clojure.lang.PersistentArrayMap"))
(def clojure.lang.PersistentListMap clojure.lang.PersistentArrayMap)

(def clojure.lang.Macro (clojure.interop/deftype-rs "clojure.lang.Macro"))

(def clojure.lang.ISeq (clojure.interop/deftype-rs "clojure.lang.ISeq"))

(def clojure.lang.Nil (clojure.interop/deftype-rs "clojure.lang.Nil"))

(def rust.regex (clojure.interop/deftype-rs "rust.regex"))
(def java.util.regex.Pattern rust.regex)
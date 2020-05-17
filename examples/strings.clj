
(defn main []

      (println "'hello world' ends with 'orld' : " (clojure.string/ends-with? "hello world" "orld"))
      (println "'hello world' starts with 'hel' : " (clojure.string/starts-with? "hello world" "hel"))

      (println "nil is empty string : " (clojure.string/blank? nil))
      (println "whitespace is empty string : " (clojure.string/blank? " "))
      (println "hello is empty string : " (clojure.string/blank? "hello"))

      (println "hello,world splitted by regex pattern " #"\"" " " (clojure.string/split "hello,world", #","))

      (println "murder is backwards : " (clojure.string/reverse "murder"))

      (println "upper case hello : " (clojure.string/upper-case "hello"))
      (println "lower case HELLO : " (clojure.string/lower-case "HELLO"))

      (println "joining array of items with ', ' : " (clojure.string/join ", " [1, "second", true]))

      (println "hello world includes 'o wor' : " (clojure.string/includes? "hello world" "o wor")))

(main)


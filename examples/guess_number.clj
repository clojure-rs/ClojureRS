(println "Guess a number between 1 and 3.")
(print "guess > ")
(flush)
(let [answer (read-line)
      number (+ (rand-int 3) 1)]

  (if (= answer (str number))
    (println "correct!")
    (println "wrong, the correct was " number)))

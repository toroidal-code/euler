(defn fib [n]
  "Caclculates the n-th fibbonacci number"
  (defn _fib [n, a, b]
    (if (== n 0) 
      a
      (_fib (- n 1) (+ a b) a)))
  (_fib n 0 1))

(let
    [fibat (ref 1)
     fibn  (ref 0)
     sum   (ref 0)]
  (while (<= @fibn 4000000)
    (dosync
     (ref-set fibn (fib @fibat))
     (alter fibat + 1)
     (if (== 0 (mod @fibn 2))
       (alter sum + @fibn))))
  (println @sum))

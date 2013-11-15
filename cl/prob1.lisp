(reduce #'(lambda (head tail) 
            (if (or (zerop (mod tail 3)) 
                    (zerop (mod tail 5)))
                (+ head tail)
              head)) 
        (loop for x below 1000 collect x))

(apply #'+ (loop for x below 1000 
              if (or (zerop (mod x 3))
                     (zerop (mod x 5)))
              collect x))

(loop for x below 1000 
   when (or (zerop (mod x 3))
            (zerop (mod x 5)))
   sum x))

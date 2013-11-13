(reduce #'(lambda (head tail) 
			(if (or (= 0 (mod tail 3)) 
					(= 0 (mod tail 5)))
				(+ head tail)
			  head)) 
		(loop for x below 1000 collect x))

(apply #'+ (loop for x below 1000 
			  if (or (= 0 (mod x 3))
					 (= 0 (mod x 5)))
			  collect x))

(display "TEST 1: ----- make-point			(2 . 3)")
(newline)
(display (make-point 2 3))
(newline)

(display "TEST 2: ----- is-line				#t")
(newline)
(display (is-line (make-point 1 3) (make-point 2 6) (make-point 3 9)))
(newline)

(display "TEST 3: ----- is-line				#f")
(newline)
(display (is-line (make-point 2 2) (make-point 5 2) (make-point 3.5 -2)))
(newline)

(display "TEST 4: ----- distance			3.1622")
(newline)
(display (distance (make-point 1 3) (make-point 2 6)))
(newline)

(display "TEST 5: ----- perimeter			11.5440")
(newline)
(display (perimeter (make-point 2 2) (make-point 5 2) (make-point 3.5 -2)))
(newline)

(display "TEST 6: ----- area				5.9999")
(newline)
(display (area (make-point 2 2) (make-point 5 2) (make-point 3.5 -2)))
(newline)

(display "TEST 7: ----- calculate-triangle	5.9999")
(newline)
(calculate-triangle (make-point 2 2) (make-point 5 2) (make-point 3.5 -2))

(display "TEST 8: ----- Infinite Slope		#t")
(newline)
(display (is-line (make-point 2 2) (make-point 2 3) (make-point 2 -2)))
(newline)


(newline)
(newline)

(exit)

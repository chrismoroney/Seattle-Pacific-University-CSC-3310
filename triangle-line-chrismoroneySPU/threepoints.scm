; Chris Moroney
; Dr. Arias
; CSC 3310
; Due 15 November 2020
; Scheme Programming Project

; HELPER FUNCTIONS

; Defines a value for "pi", which is a number assigned to pi (more digits allowed for better precision)
(define pi 3.141592653589)

; Defines a function to square any value, which is to multiply a value x by itself
(define (square x)
  (* x x))

; A function that rounds calculated values to a certain number of decimal points.
; Normally the only scheme function there is a round function, but that doesn't help us find
; particular decimal points. Instead, we can multiply our value by a power of 10, round that
; value, and then divide by the same power of 10. The power of 10 is determined by how many
; decimal places we want to round our answer to.
(define (round-up-down value decimal-places)
  (let ((power-of-ten (expt 10 decimal-places)))
    (/ (round (* power-of-ten value)) power-of-ten)))
; NOTE that I found a version of this type of function on stack overflow and "programming-forum".
; I varied this function slightly in order to integrate it into my code.

; A function that calculates the slope between two points. We assign get the x and y values
; from both points, and calculate the slope by subtracting the y-values from each other
; and dividing it by the difference of the x-values.
(define (get-slope pointA pointB)
(let ((x1 (get-x pointA))
  (x2 (get-x pointB))
  (y1 (get-y pointA))
  (y2 (get-y pointB)))
  ; If both x-values are equal (or the difference is 0), then we have division by 0,
  ; so we will just set the slope to a very large number.
  (if (= x2 x1)
    999999999
  ; Otherwise we just calculate the slope as normal
  (/ (- y2 y1) (- x2 x1)))))

; A function to calculate the angle of a triangle. In order to calculate the angle,
; we need to find the length of each of the sides, then we will use the law of
; cosines to find the angle. If the law of cosines says a^2 = b^2 + c^2 - 2bc*cos(A),
; then we need to solve for A by using equation acos((a^2 - b^2 - c^2) / (-2bc))
(define (angle pointA pointB pointC)
  (let ((lineA (distance pointA pointB))
  (lineB (distance pointB pointC))
  (lineC (distance pointC pointA)))
  (acos (/ (- (square lineA) (square lineB) (square lineC)) (* -2 lineB lineC)))))

; END OF HELPER FUNCTIONS

; REQUIRED FUNCTIONS

; We define a point by making a "list" with the cons function, which takes in two numbers
; as an x-value and y-value
(define (make-point x-cor y-cor)
  (cons x-cor y-cor))

; This function is a 'getter' to get the x-value of a point, which can be extracted
; with the car function, grabbing the first value in a list
(define (get-x point)
  (car point))

; This function is a 'getter' to get the y-value of a point, which can be extracted
; with the cdr function, grabbing the remaining value or values in a list
(define (get-y point)
  (cdr point))

; This function determines if there is a line that runs through 3 points. We know
; there is a line that runs through all 3 points if the slope between points A and B
; is the same as the slope between B and C
(define (is-line pointA pointB pointC)
  (let ((slope1 (get-slope pointA pointB))
  (slope2 (get-slope pointB pointC)))
  (= slope1 slope2)))

; This function determines the distance between 2 points. We get the x and y values
; for both points, and then find the difference between the x values and y values. We
; sum those values together, and then take the squareroot.
(define (distance pointA pointB)
  (let ((x1 (get-x pointA))
  (x2 (get-x pointB))
  (y1 (get-y pointA))
  (y2 (get-y pointB)))
  (sqrt (+ (square (- x2 x1)) (square (- y2 y1))))))

; This function finds the perimeter of a triangle, which is just the sum of the side
; lengths of the triangle.
(define (perimeter pointA pointB pointC)
  (let ((lineA (distance pointA pointB))
  (lineB (distance pointB pointC))
  (lineC (distance pointC pointA)))
  (+ lineA lineB lineC)))

; This function finds the area of a triangle. With the 3 side lengths, we can calculate
; the area of a triangle to be sqrt(s*(s-a)*(s-b)*(s-c)) where a, b, and c the sides of a triangle,
; and s is the sum of a, b, and c all divided by two.
(define (area pointA pointB pointC)
  (let ((lineA (distance pointA pointB))
  (lineB (distance pointB pointC))
  (lineC (distance pointC pointA))
  (S (/ (perimeter pointA pointB pointC) 2)))
  (sqrt (* S (- S lineA) (- S lineB) (- S lineC)))))

; This function displays the information of a triangle given threepoints.
; This includes the three side lengths, perimeter, area, and angles of the triangle.
; Here, we also use the rounding function to determine how many decimal places we
; want to round to. For the angles, the default value is in radians, so the second
; set of values in the angles represent the angle in degrees.
(define (calculate-triangle pointA pointB pointC)
  (newline)
  (display "Side 1 = ")
  (display (round-up-down (distance pointA pointB) 3))
  (newline)
  (display "Side 2 = ")
  (display (round-up-down (distance pointB pointC) 3))
  (newline)
  (display "Side 3 = ")
  (display (round-up-down (distance pointC pointA) 3))
  (newline)
  (display "Perimeter = ")
  (display (round-up-down (perimeter pointA pointB pointC) 3))
  (newline)
  (display "Area = ")
  (display (round-up-down (area pointA pointB pointC) 3))
  (newline)
  (display "Angle 1 = ")
  ; radians
  (display (round-up-down (angle pointA pointB pointC) 5))
  (display "   ")
  ; convert radians to degrees by multiplying by 180/pi
  (display (round-up-down (* (angle pointA pointB pointC) (/ 180 pi)) 5))
  (newline)
  (display "Angle 2 = ")
  ; radians
  (display (round-up-down (angle pointB pointC pointA) 5))
  (display "   ")
  ; convert radians to degrees by multiplying by 180/pi
  (display (round-up-down (* (angle pointB pointC pointA) (/ 180 pi)) 5))
  (newline)
  (display "Angle 3 = ")
  ; radians
  (display (round-up-down (angle pointC pointA pointB) 5))
  (display "   ")
  ; convert radians to degrees by multiplying by 180/pi
  (display (round-up-down (* (angle pointC pointA pointB) (/ 180 pi)) 5))
  (newline)
  (newline))

; END OF REQUIRED FUNCTIONS

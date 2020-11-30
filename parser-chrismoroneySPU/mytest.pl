/* processing input file test.cpl
   Lexical and Syntax analysis passed */
query(line(point2d(1,10), point2d(7,10), point2d(5,20))).
query(triangle(point2d(1,10), point2d(7,10), point2d(5,20))).
query(vertical(point2d(1,10), point2d(7,10))).
query(vertical(point2d(1,10), point2d(5,20))).
query(vertical(point2d(7,10), point2d(5,20))).
query(horizontal(point2d(1,10), point2d(7,10))).
query(horizontal(point2d(1,10), point2d(5,20))).
query(horizontal(point2d(7,10), point2d(5,20))).
query(equilateral(point2d(1,10), point2d(7,10), point2d(5,20))).
query(isosceles(point2d(1,10), point2d(7,10), point2d(5,20))).
query(right(point2d(1,10), point2d(7,10), point2d(5,20))).
query(scalene(point2d(1,10), point2d(7,10), point2d(5,20))).
query(acute(point2d(1,10), point2d(7,10), point2d(5,20))).
query(obtuse(point2d(1,10), point2d(7,10), point2d(5,20))).
writeln(T) :- write(T), nl.
main:- forall(query(Q), Q-> (writeln('yes')) ; (writeln('no'))),
  halt.

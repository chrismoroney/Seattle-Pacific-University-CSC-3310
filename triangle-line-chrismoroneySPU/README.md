# Scheme Programming Assignment
## Motivation
Around 1930’s one mathematical model of computation was created called the Lambda Calculus, with time it was proven to be equivalent to the Turing Machine model, meaning that both models have the same computational capability. Functional programming is an elaboration based on Lambda Calculus, where programming is declarative as opposed to imperative as most programming languages students have been exposed so far. At a point in the development of Computer Science, functional programming was thought to be applicable to solve AI problems. Besides being an academic type of programming, industry also uses functional programming, for instance Erlang is used in the telecommunications industry. There are modern functional programming languages like Scala and F#.
In this assignment, students will develop simple programs in Scheme in order to practice this language and expose them to a completely different paradigm of programming. Up to this moment, most of the students have been working and coding with imperative and object oriented languages, and a declarative language should pose a new challenge, as it requires a different way to think.
## Description
Write a Scheme program that implements the following functions:
- `(make-point x-cor y-cor)`. This function needs to create a “list” that will have two elements: `x-cor` and `y-cor`. You will need to use the cons function. Additionally, it is suggested that you make a `(get-x point)` and a `(get-y point)` functions, using the `car` and `cdr` functions to retrieve the x and y values of a point.
- `(is-line point1 point2 point3)`. Using the created points (using `make-point`) this function returns true (`#t`) if the three points form a line or false (`#f`) if they form a triangle.
- `(distance point1 point2)`. Calculates the distance between two points
- `(perimeter point1 point2 point3)`. Calculates the perimeter of a triangle defined by the three points
- `(area point1 point2 point3)`. Calculates the area of a triangle defined by the three points
- `(calculate-triangle point1 point2 point3)`. Calculates the perimeter, area and interior angles of the triangle formed by the three points.

## Assignment Requirements
- Good programming practices
- Correct and readable indentation
- This is an strictly individual assignment

## Delivery Method
You will push your final version to the repository of the assignment before the deadline, your program must be in a file named `threepoints.scm`  **[You MUST name your program this, failure will result in zero grade]**

## Assessment and Grading
Assessment will consider the following factors in the grading of the project:
-	Adherence to instructions
-	Correct function of the program
-	No runtime errors
-	Late deliveries will have a zero mark
-	Plagiarism will have a double zero mark (in addition to losing 10% of your final grade, the group that plagiarizes will lose an additional 10% of their final grade), besides there will be a report filed in the students’ academic record.
-	Each program will be loaded in Scheme and test it to check if the functions are in working order.

## Sample Output
```
scheme-assignments arias$ scheme -load threepoints.scm
MIT/GNU Scheme running under OS X
Type `^C' (control-C) followed by `H' to obtain information about interrupts.

Copyright (C) 2014 Massachusetts Institute of Technology
This is free software; see the source for copying conditions. There is NO warranty; not even for MERCHANTABILITY or
FITNESS FOR A PARTICULAR PURPOSE.

Image saved on Saturday May 17, 2014 at 2:39:25 AM
  Release 9.2 || Microcode 15.3 || Runtime 15.7 || SF 4.41 || LIAR/x86-64 4.118 || Edwin 3.116
;Loading "threepoints.scm"... done

1 ]=> (make-point 2 3)

;Value 13: (2 . 3)

1 ]=> (is-line (make-point 1 3) (make-point 2 6) (make-point 3 9))

;Value: #t

1 ]=> (distance (make-point 1 3) (make-point 2 6))

;Value: 3.1622776601683795

1 ]=> (perimeter (make-point 2 2) (make-point 5 2) (make-point 3.5 -2))

;Value: 11.54400374531753

1 ]=> (area (make-point 2 2) (make-point 5 2) (make-point 3.5 -2))

;Value: 5.999999999999999

1 ]=> (calculate-triangle (make-point 2 2) (make-point 5 2) (make-point 3.5 -2))
Side 1 = 3
Side 2 = 4.272
Side 3 = 4.272
Perimeter = 11.544
Area = 6.
Angle 1 = .71754   41.11209
Angle 2 = 1.21203   69.44395
Angle 3 = 1.21203   69.44395

;Unspecified return value

1 ]=>
```

## Instructions for Testing
A file named `test.scm` is provided. You will need to append this file to the end of your code, create a new file called `full.scm` that contanis your code and then the code from `test.scm`, and then run Scheme from the command line:  

`scheme --load full.scm > your-output.txt`

***You may need to press the y key to indicate that you want to quit***.

The command tells Scheme to run the file `full.scm`. Once the process is finished, there will be a file named `your-output.txt` you can then compare this file with another file that is provided: `output.txt` to check if your results are correct. Before turning in your assignment, you must delete the test appended lines in your code, your file `threepoints.cpp` should not have test code.

### Steps
1. Copy your `threepoints.scm` file into a new empty file called `full.scm`
1. Append the file `test.scm` at the end of your code of `full.scm`
   1. Optionally, if you are comfortable using command line: `cat threepoints.scm test.scm > full.scm`. This will concatenate your file with the test file and create a new file called `full.scm`.
1. Run the following command:
    `scheme --load full.scm > your-output.txt`
1. Compare the file `your-ouput.txt` with the provided `output.txt`, you may use `diff` or `fc` to compare the files.
1. If your output matches, then you are ready to push the final version of your work
1. Push the repository  

## Recommendation

### Mac Users
Install Prolog using Brew: `brew install mit-scheme`. Installing the DMG produced too many problems for me.

### Windows Users
- [Install the Windows Linux Subsystem](https://docs.microsoft.com/en-us/windows/wsl/install-win10)
- [Install Ubuntu interface from the Windows Store](https://www.microsoft.com/en-us/p/ubuntu-1804-lts/9n9tngvndl3q?rtc=1&activetab=pivot:overviewt)
- Once you are in your Ubuntu terminal:
  - `sudo apt-get update`
  - `sudo apt-get install mit-scheme`

### GUI Versions
There are GUI versions for the Scheme Interpreter, but they might not help you with the assignment goals. Use them at your own risk.

## About the Parser

Steps to integrate the parser made in Rust:
1. Create a 3Points Language "program" call it `test.cpl`, define three points.
1. Run your parser: `cargo run test.cpl -s > mytest.scm`
1. Paste your generated test to your scheme program: `cat threepoints.scm mytest.scm > full.scm`
1. Run your program to check what those three points in `test.cpl` represent: `scheme --load full.scm`

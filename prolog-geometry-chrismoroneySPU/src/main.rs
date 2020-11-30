// Chris Moroney
// Professor Arias
// Seattle Pacific University
// CSC 3310
// Due 26 October 2020 11:59PM

// ********* 3 POINT GRAMMAR RUST ASSIGNMENT ********* //

// NOTE: SOME println!() lines are for spacing and formatting in the terminal, and some are commented
// out for printing, such as the list of tokens in a file (mainly in char_check function).

// NOTE: lines 169, 176, 197, and 205 prevent printing out of all tokens. If desired, uncomment these
// lines to see the list of tokens produced

// ************* LIBRARIES AND PACKAGES ************** //

// Module that allows for manipulation of the rust environment
use std::env;
// Allows for program to use String data type
use std::string::String;
// Allows for program to use open file and read in a file from the system
use std::fs::File;
// Assists in importing packages to I/O
use std::io::prelude::*;
// Allows for program to use Regular Expressions (Cargo.toml now has dependency regex = "1")
use regex::Regex;

// ********* END OF LIBRARIES AND PACKAGES *********** //


// ******************* MAIN PROGRAM ****************** //

fn main() -> std::io::Result<()>{
    // Initialize a Vector of strings for each of the args (directory, file name, flag)
    let args: Vec<String> = env::args().collect();
    // Initialize a Vector of numbers that will be recieved from the "nums" when checking for
    // the lexemes in the char_check function
    let _nums: Vec<String> = [].to_vec();
    // If the number of args is correct at 3
    if args.len() == 3 {
        // Open the file that was inputted by the user (should be arg[1] if entered correctly)
        let mut file = File::open(&args[1])?;
        // Initialize a mutable string which will read the contents of the file.
        let mut contents = String::new();
        // Reads the contents of the file and puts the contents into "contents" as a string
        file.read_to_string(&mut contents)?;
        // Assigning a bool and a vector of numbers (check and _nums) to the result of checking our contents
        // Thus, we need to run the char_check method and pass in our string read from the file inputted
        let (check, _nums) = char_check(contents);
        // If our string passes the Lexical and Syntax analysis, then we need to check what type of
        // output we are going to print, based off of the inputted tag
        if check == true {
            // If the flag inputted is "-s"...
            if args[2] == "-s" {
                // Print out the following for our Scheme input
                println!("; processing input file {}", args[1]);
                println!("; Lexical and Syntax analysis passed");
                // Proceed to print out the output with the print_scheme function, passing in our array
                // of nums passed back from the char_check
                print_scheme(_nums);
            // Else if the flag inputted is "-s"...
            } else if args[2] == "-p" {
                // Print out the following for our Prolog input
                println!("/* processing input file {}", args[1]);
                println!("   Lexical and Syntax analysis passed */");
                // Proceed to print out the output with the print_prolog function, passing in our array
                // of nums passed back from the char_check
                print_prolog(_nums);
            // We only should be coming here if the inputted flag is not -s or -p
            } else {
                // Prints these lines to show that the incorrect flag was used.
                println!("Processing input file {}", args[1]);
                println!("Lexical and Syntax analysis passed");
                println!("Unknown flag inputted (expecting '-s' or '-p')");
            }
        }
    // Else if the user inputs too few args necessary (requiring 2 inputs, directory already counts as one)
    } else if args.len() < 3 {
        // Doesn't run, shows user what to input
        println!("Couldn't process, not enough parameters (requires: 'cargo run (file) (flag)')");
        println!();
    // Else, the user inputted too many parameters
    } else {
        // Doesn't run, shows user what to input
        println!("Too many parameters (requires: 'cargo run (file) (flag)')");
        println!();
    }
    // When we reach this point, we indicate to the program that regardless of the outcome, it was a success
    Ok(())
}

// *************** END OF MAIN PROGRAM *************** //


// ****** LEXICAL AND SYNTAX ANALYSIS FUNCTION ****** //

// This function checks the file for any lexical or syntax errors, as well as processes
// the string from the file to be easily ready and passed back to the main function for proper printing.
// We input the string from our inputted file, and return both a bool to indicate whether or not there is
// an error in the program or not, as well as a Vector of strings to return the numbers we gathered in our
// program.
// NOTE: The returned Vector of Strings is representing the numbers in the program. The numbers themselves
// are strings or chars because we are reading each char from the file as a string, which means each individual digit
// is a char, not an int. Thus, we must make a vector of strings in order to place our "chars" inside the
// vector. We keep the numbers as a string because there could be multiple digits, and we are checking
// each individual char in the string during our lexical analysis.
fn char_check(contents: String) -> (bool, Vec<String>) {
    // Take the string inputted and remove all whitespaces. Spaces are not included in our grammar.
    let contents = contents.replace(" ", "");
    // Take the string inputted and remove all new lines. New lines are not included in our grammar.
    let contents = contents.replace("\n", "");
    // Initialize a token label that is blank
    let mut token = "";
    // Initialize a variable name in order to keep track of our IDs and NUMS when we are printing
    // out our TOKENS (name of variable needed for these two tokens)
    let mut var_name: String = "".to_string();
    // Initialize and empty Vector that will collect our nums and send it back to main in order
    // to help with printing the scheme or prolog lines.
    let mut return_nums: Vec<String> = [].to_vec();

    // **************** LEXICAL ANALYSIS **************** //

    // NOTE: Println!() statements are commented out because  I was testing all of
    // the declared TOKENS printed in a list.

    // Starting out the sequence of tokens in the string inputted.
    //println!("TOKENS");
    //println!();
    // For each character in the string inputted from the file
    for c in contents.chars() {
        // Check if the char is alphabetical. If it is...
        if c.is_alphabetic() {
            // Assign the token to ID.
            token = "ID";
            // Push the char into the variable name. This is necessary for collecting the full name
            // of the variable when we print out the TOKENS used for this particular file.
            var_name.push(c);
        // If it's not alphabetical, check if its a number. If it is...
        } else if c.is_numeric(){
            // Assign the token to NUM
            token = "NUM";
            // Push the char into the variable name. This is necessary for collecting the full name
            // of the variable when we print out the TOKENS used for this particular file.
            var_name.push(c);
        // If its neither of those listed above, that means we have a symbol
        // NOTE: We reach here when the CURRENT char is a symbol. We should have a collection of either
        // letters or digits that we have been collecting, and we will either try to find a POINT token
        // or simply just print the collection of chars or digis we have been STORING in var_name.
        } else {
            // If the CURRENT token we are on is a NUM (hasn't changed since checking the 'next' char)...
            if token == "NUM" {
                // Put the variable name into sent_name
                let sent_name = &var_name;
                // Push sent name into the return array, we are keeping this number for later (print...)
                return_nums.push(sent_name.to_string());
                // NOTE: We can't put var_name into the return_nums because there will be a scoping issue.
                // We also cannot do it reference the variable with & due to the variable already moving once.
                // Thus, we simply have to make a new variable and put the existing variable in that, then move
                // the new variable.
            }
            // If the current variable name has chars or digits (not empty)...
            // NOTE: Although vague, this helps prevent certain tokens being printed without chars
            // or digits before this current char. We want to make sure all the previous chars are printed
            // because all chars or series of chars or digits should be before a symbol.
            if var_name.len() != 0 {
                // check if the letters that are currently stored in var_name is 'point'. If it is point...
                if var_name == "point"{
                    // our new token is POINT, instead of ID point
                    //token = "POINT";
                    // Since we have token POINT, we do not want the var_name anymore, so just erase
                    var_name = var_name.replace(&var_name, "");
                }
                // Here, we simply print out our token, which would be either ID, NUM, or POINT,
                // along with any value we have inside of var_name. This represents printing
                // one token that we have found that is either an ID, NUM, or POINT.
                //println!("{} {}", token, &var_name);

                // clear the var_name of stored chars or digits, prepare for next ID, NUM, or POINT.
                var_name = var_name.replace(&var_name, "");
            }
            // NOW we finally focus on the CURRENT char by matching the CURRENT char to this
            // directory of lexemes to tokens.
            // The possible TOKENS are SEMICOLON, COMMA, PERIOD, LPAREN, RPAREN, ASSIGN, and UNKNOWN
            match c {
                ';' => token = "SEMICOLON",
                ',' => token = "COMMA",
                '.' => token = "PERIOD",
                '(' => token = "LPAREN",
                ')' => token = "RPAREN",
                '=' => token = "ASSIGN",
                // default for unknown is _. Represents "all other cases"
                _ => token = "UNKNOWN"
            }
            // if we find a token that is UNKNOWN or POINT...
            if token == "UNKNOWN" {
                // Print the TOKEN out
                //println!("{}", token);
                // declare error
                println!("Lexical error detected");
                // immediately return false and the current numbers. This will stop searching and
                // end the program
                return (false, return_nums);
            }
        // In all cases other than UNKNOWN for symbols, we want to print out the given TOKEN.
        //println!("{}", token);
        }
    }

    // ************ END OF LEXICAL ANALYSIS ************ //


    // **************** SYNTAX ANALYSIS **************** //

    // Creating a STRING that represents our regular expression to determine what type of grammar the
    // sentence should follow. Our string from the inputted file is required to have an alphabetical
    // name (one or more letters), followed by an equals sign and the word "point". We then need a
    // left parentheses symbol, followed by two integers of any length, separated by a comma in the
    //  middle. We then will provide a right parentheses symbol and a semicolon. We will repeat this
    // two more times, where the second time is exactly the same as the first time, and the last
    // time is exactly the same, except we replace the semicolon with a period symbol.
    let myregex = r"^[a-z]+=point\(\d+,\d+\);[a-z]+=point\(\d+,\d+\);[a-z]+=point\(\d+,\d+\)\.$";
    // We will make the actual regular expression here, using the STRING (myregex) above.
    let re = Regex::new(myregex).unwrap();
    // if the string from the inputted file does not follow our regular expression...
    if !re.is_match(&contents) {
        // We have a syntax error. Let user know that there is a syntax error.
        println!("Syntax error detected");
        // return immediately to main as false that the string has failed the Syntax test.
        return (false, return_nums);
    }

    // ************ END OF SYNTAX ANALYSIS ************ //

    // We will ONLY reach this point if we did not return false during either the
    // LEXICAL ANALYSIS or SYNTAX ANALYSIS. If we reach this point, that means the string from the
    // inputted file has passed, and we can tell main that the tests have passed.
    // Returns true for completing the Lexical and Syntax Analysis, as well as the numbers that
    // we had collected from the NUMS during the LEXICAL ANALYSIS.
    return (true, return_nums);
}

// *** END OF LEXICAL AND SYNTAX ANALYSIS FUNCTION **** //


// *************** PRINT SCHEME FUNCTION ************** //

// If the user used the -s flag as part of their inputted parameters, then we come to here and output
// the line of Scheme desired. We take the numbers from Lexical analysis in the char_check function
// and insert the numbers gathered here.
fn print_scheme(nums: Vec<String>) {
    println!("(calculate-triangle (make-point {} {}) (make-point {} {}) (make-point {} {}))", nums[0], nums[1], nums[2], nums[3], nums[4], nums[5]);
}

// *********** END OF PRINT SCHEME FUNCTION *********** //


// *************** PRINT PROLOG FUNCTION ************** //

// If the user used the -p flag as part of their inputted parameters, then we come to here and output
// the line of Prolog desired. We take the numbers from Lexical analysis in the char_check function
// and insert the numbers gathered here.
fn print_prolog(nums: Vec<String>) {
    println!("query(line(point2d({},{}), point2d({},{}), point2d({},{}))).", nums[0], nums[1], nums[2], nums[3], nums[4], nums[5]);
    println!("query(triangle(point2d({},{}), point2d({},{}), point2d({},{}))).", nums[0], nums[1], nums[2], nums[3], nums[4], nums[5]);
    println!("query(vertical(point2d({},{}), point2d({},{}))).", nums[0], nums[1], nums[2], nums[3]);
    println!("query(vertical(point2d({},{}), point2d({},{}))).", nums[0], nums[1], nums[4], nums[5]);
    println!("query(vertical(point2d({},{}), point2d({},{}))).", nums[2], nums[3], nums[4], nums[5]);
    println!("query(horizontal(point2d({},{}), point2d({},{}))).", nums[0], nums[1], nums[2], nums[3]);
    println!("query(horizontal(point2d({},{}), point2d({},{}))).", nums[0], nums[1], nums[4], nums[5]);
    println!("query(horizontal(point2d({},{}), point2d({},{}))).", nums[2], nums[3], nums[4], nums[5]);
    println!("query(equilateral(point2d({},{}), point2d({},{}), point2d({},{}))).", nums[0], nums[1], nums[2], nums[3], nums[4], nums[5]);
    println!("query(isosceles(point2d({},{}), point2d({},{}), point2d({},{}))).", nums[0], nums[1], nums[2], nums[3], nums[4], nums[5]);
    println!("query(right(point2d({},{}), point2d({},{}), point2d({},{}))).", nums[0], nums[1], nums[2], nums[3], nums[4], nums[5]);
    println!("query(scalene(point2d({},{}), point2d({},{}), point2d({},{}))).", nums[0], nums[1], nums[2], nums[3], nums[4], nums[5]);
    println!("query(acute(point2d({},{}), point2d({},{}), point2d({},{}))).", nums[0], nums[1], nums[2], nums[3], nums[4], nums[5]);
    println!("query(obtuse(point2d({},{}), point2d({},{}), point2d({},{}))).", nums[0], nums[1], nums[2], nums[3], nums[4], nums[5]);
    println!("writeln(T) :- write(T), nl.");
    println!("main :- forall(query(Q), Q-> (writeln('yes')) ; (writeln('no'))),");
    println!("  halt.");
}

// *********** END OF PRINT PROLOG FUNCTION *********** //

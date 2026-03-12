################  a ###################
1# Sums

* Write a function that receives an array of numbers, and returns an index `i` into the array, such that the sum of numbers with index smaller than `i` equals to the sum of numbers with an index greater than `i`. If no such position exists, return -1.

---

2# Calculator

* Write a program that receives an operator (`+`/`-`/`*`/`/`) and two numbers, and prints the computation output. e.g.:

```
$ cargo run -- 2 + 3
Answer: 5
```

---

3# Multiplication

* Write a program that receives N numbers (N is user-specified) and outputs a multiplication table of the numbers by themselves. i.e. - a table with the provided numbers as both row and column headers, where each cell is the result of multiplying its column header by its row header

```
$ cargo run -- 1 2 3 4 5 6
  1 2 3 4 5 6
1 1 2 3 4 5 6
2 2 4 6 8 10 12
3 3 6 9 12 15 18
4 4 8 12 16 20 24
...
```

---
4# Reverse arrays

* Write a function, `reverse` that reverses a sequance of numbers (array or vector) by returning a reversed copy of it
* Write a function, `reverse_in_place` that reverses an array/vector in-place
* Write a function, `reverse_str` that returns a reversed copy of a string

* **NOTES**
    * Performance is not a requirement for now
    * Use unit tests to make sure your code follows the requirements
---
5# Summary

Write a function receiving a 2-dimensional list of numbers (of the form `Vec<Vec<u32>>`). You can assume all rows have same length, and all columns have the same length. Your program should append another cell at the "end" of each row and column, summarizing that row or column (i.e. a "totals" column and a "totals" row)

---

6# RLE

* Write a program that receives a sequence of numbers, and produces an "RLE compressed" version of the numbers, as follows:
  * Each sequence of identical numbers in the input gets replaced with its count followed by the identical number in the output
  * Numbers that are not part of an identical sequence should be considered as a "sequence of 1 item"

```
$ cargo run -- 1 1 1 2 4 10 10 10 10 10
3 1 1 2 1 4 5 10
```

7### Card battle

* Write code that models a regular card deck (A, 2-10, J, Q, K of all four suites)
* Write a function that receives two decks of cards of equal length, and decides who would win when the following game is played:
  * On each turn both contestants draw the top card from their decks, and the highest card wins. 
  * The highest card is determined according to the number of the card (Ace is lowest, then 2-10, then J, Q and K)
    * Assuming both cards have the same number, the holder of the highest suite wins (order is Clubs < Diamonds < Hearts < Spades)


8### RuleSet

* We would like to construct a data type that represents a "rule" used to filter strings. The end result of a filter check is whether or not a specific string matched the filter. A single filter can be one of:
  * `Substr(s)` - returns true if `s` is contained in the given string
  * `Startswith(s)` - returns true if the string starts with a prefix `s`
  * `Endswith(s)` - same, but for suffix
  * `Length(n)` - returns true if the string's length is at least `n`
  * `And(preds)` - connects several predicates with an `and` relationship (returning true only if all of them return true)
  * `Or(preds)` - you guessed it


### Drawing Board

* We are going to implement a drawing engine
* The engine maintains a canvas on top of which lines are drawn, in various colors
* The engine has a current position (x, y coords) and a current color.
* You need to process a batch of commands. Each command can be either:
    * Move to a new position (x, y)
    * Set the current color to a new color
    * Print the lines that have already been drawn on the canvas


9### Bank

* We will write a simulator for a bank with multiple customers. It will consist of one struct implementing the bank, and one struct implementing a customer.
* Customers will contain their identity (first name, last name), and once added to the bank will also contain its internal id in the bank systems
  * Note that the type for "customer" does not have to be the same type for a "customer in the bank". Design the interface as you see fit
* The bank will support the following interface:
  * Initialize the bank with an initial amount of money in its "vault", and the maximum debt a customer can get into
  * Add a customer to the bank, returning its id in the bank records
  * Add funds to the vault
  * Deposit an amount of money to a specific customer's account
  * Withdraw an amount of money from a customer's account
    * This function should return a value indicating whether or not the withdrawal is allowed
  * Borrow money under a specific customer's account
    * Borrowing money transfers money from the bank's vault to the customer's account
    * This function should return whether or not the borrowing is allowed (borrowing is not allowed if the customer already exceeded the debt limit or if the bank's vault has insufficient funds)
  * Return a specific amount of borrowed money for a specific customer
    * Deduces the amount from the customer's balance, and puts it back into the bank's vault
  * **NOTE**: You don't have to handle all corner cases for now (except for the specific conditions mentioned above that you should detect)


  ###############  b #############
**NOTE**: All solutions should be properly modularized (logic in one module, `main` in its own module)


# cat
Write a program that outputs the contents of a file, provided as a command-line argument (a.k.a `cat`)

# tac
Write a program that outputs the content of a file, only in reversed line order

# cut
Write a program that receives an input file and a list of field numbers, and outputs only these fields from the file lines. For example, given this file:
```
line1_f1 line1_f2 line1_f3
line2_f1 line2_f2 line2_f3
line3_f1 line3_f2 line3_f3
```
Running `./prog file.txt 1 3` will output:

```
line1_f1 line1_f3
line2_f1 line2_f3
line3_f1 line3_f3
```

# grep
Write a program that receives a filename and a pattern, and outputs only lines of the file containing the pattern (a.k.a `grep`)

# ls
Write a program the outputs the listing of the directory provided in the command-line (hint: look up `std::fs::read_dir`)

# find
Write a program that receives a name of a file and a directory. The program should traverse the directory recursively and print any path in which the given file name was found (the file's full path should be printed)

Hint: use the example in `std::fs::read_dir`'s docs

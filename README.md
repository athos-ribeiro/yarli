## Yet Another Rust Lox Implementation

This is a Rust implementation of the Lox language introduced in first part of
[craftinginterpreters.com](http://craftinginterpreters.com/) (yarli is a
tree-walk interpreter).

This is **NOT** intended for production (nor any kind of) use. This is just a
fun project used to learn Rust.

This is far from complete. The list below is intended to track the project
progress, Once it is complete, we should have a working Lox implementation as
discribed in the book.

* [X] Chapter 04: Scanning
* [X] Chapter 05: Representing Code
* [ ] Chapter 06: Parsing Expressions
* [ ] Chapter 07: Evaluating Expressions
* [ ] Chapter 08: Statements and State
* [ ] Chapter 09: Control Flow
* [ ] Chapter 10: Functions
* [ ] Chapter 11: Reserving and Binding
* [ ] Chapter 12: Classes
* [ ] Chapter 13: Inheritance

### Development and Usage

You can run this project with the usual cargo workflow.
`yarli` runs either in an interactive mode (just run `yarli`) or by running
scripts through `yarli FILE_PATH`.

The syntax for writing Lox scripts is available at
http://craftinginterpreters.com/the-lox-language.html

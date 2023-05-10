# Task Specification: Command Line Calculator in Rust

## Overview

Create a command line calculator written in Rust that can perform basic arithmetic operations like addition, subtraction, multiplication, and division. The program should accept user input in the form of mathematical expressions and return the result. The source code for this project should be shared on GitHub.

## Requirements

1. **Language**: The calculator should be written in Rust.
2. **Operations**: The calculator should support the following basic arithmetic operations:
   - Addition (+)
   - Subtraction (-)
   - Multiplication (\*)
   - Division (/)
3. **Input**: The calculator should accept user input as a command line argument in the form of a mathematical expression. The expression should contain numbers and supported arithmetic operators, e.g., `5 + 3 * 2`.
4. **Output**: The calculator should evaluate the expression and return the result to the user.
5. **Error Handling**: The calculator should handle invalid expressions and display a helpful error message to the user, e.g., `Invalid expression. Please use numbers and supported arithmetic operators.`.
6. **GitHub**: Share the source code on GitHub under an open-source license. Include a README.md file with instructions on how to build and run the calculator, as well as examples of usage.

## Deliverables

1. A GitHub repository containing:
   - The Rust source code files
   - A README.md file with instructions on how to build and run the calculator, as well as examples of usage
   - A `LICENSE` file containing the open-source license information
2. A working command line calculator that fulfills the requirements listed above.

## Example Usage

```sh
$ cargo build --release
$ ./target/release/calculator "5 + 3 * 2"
```

Output:

```
Result: 11
```

## Evaluation Criteria

 - The code is written in Rust and shared on GitHub
 - The calculator supports the specified arithmetic operations
 - The calculator accepts user input as a command line argument
 - The calculator returns the correct result for valid expressions
 - The calculator handles invalid expressions and provides a helpful error message
 - The README.md file contains clear instructions and usage examples

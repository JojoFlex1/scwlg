# Simple Calculator using Logic Gates
using little bit of nand2tertis idea

This is a simple calculator implemented in Rust that uses fundamental logic gates (AND, OR, XOR) to perform arithmetic operations. The calculator handles addition, subtraction, multiplication, and division by simulating how these operations are performed at the hardware level using basic digital logic.

## Overview

The calculator operates by converting decimal input into binary, performing arithmetic operations using simulated logic gates, and then converting the binary result back to decimal for display. This approach demonstrates how basic arithmetic can be implemented using binary logic, similar to how a digital circuit works.

### Features

- **Addition** using half-adder and full-adder logic.
- **Subtraction** using binary subtraction with borrow.
- **Multiplication** using binary multiplication logic.
- **Division** using binary long division.

### Logic Gates

The calculator uses fundamental logic gates to perform operations:

- **AND Gate (`and_gate`)**: Returns `1` if both inputs are `1`; otherwise, `0`.
- **OR Gate (`or_gate`)**: Returns `1` if at least one input is `1`; otherwise, `0`.
- **XOR Gate (`xor_gate`)**: Returns `1` if the inputs are different; otherwise, `0`.

### Adders

The adders use logic gates to perform binary addition:

- **Half Adder (`half_adder`)**: Adds two single bits and produces a sum and a carry bit.
- **Full Adder (`full_adder`)**: Adds three bits (two operands and a carry from a previous addition) to produce a sum and a new carry.

### Arithmetic Operations

1. **Addition (`multi_bit_addition`)**:
   - Uses the `full_adder` to add multi-bit binary numbers.
   
2. **Subtraction (`binary_subtraction`)**:
   - Simulates binary subtraction by borrowing, similar to how subtraction is performed manually.

3. **Multiplication (`binary_multiplication`)**:
   - Uses shift and add operations to multiply binary numbers.

4. **Division (`binary_division`)**:
   - Implements binary long division to calculate the quotient and remainder.

### How It Works

1. **Input Parsing**: The program reads a mathematical expression from the user (e.g., `2+3`, `4*5`, `6-2`, `8/4`).
2. **Binary Conversion**: Converts the input decimal numbers to binary.
3. **Perform Operation**: Based on the operator (`+`, `-`, `*`, `/`), the appropriate binary arithmetic function is called.
4. **Binary to Decimal Conversion**: Converts the binary result back to decimal.
5. **Output**: Displays the result.

### Example Usage

Run the program and enter an expression like `5+3`. The program will perform the following steps:

- Convert `5` and `3` to binary (`101` and `011` respectively).
- Use `multi_bit_addition` to add these binary numbers.
- Convert the binary result back to decimal.
- Display the result (`8`).

### Code Structure

- **Logic Gates Functions**:
  - `and_gate(a, b)`: Performs bitwise AND operation.
  - `or_gate(a, b)`: Performs bitwise OR operation.
  - `xor_gate(a, b)`: Performs bitwise XOR operation.
  
- **Adder Functions**:
  - `half_adder(a, b)`: Computes the sum and carry for a single bit addition.
  - `full_adder(a, b, carry_in)`: Extends the half adder to include a carry input.
  
- **Arithmetic Functions**:
  - `multi_bit_addition(a, b)`: Adds two binary numbers.
  - `binary_subtraction(a, b)`: Subtracts one binary number from another.
  - `binary_multiplication(a, b)`: Multiplies two binary numbers.
  - `binary_division(a, b)`: Divides one binary number by another.

- **Helper Functions**:
  - `parse_expression(input)`: Parses user input to extract numbers and operator.
  - `decimal_to_binary_vec(num)`: Converts a decimal number to its binary representation in a vector.

### Getting Started

To run the program, ensure you have Rust installed on your system. Then, clone the repository and run:

```bash
cargo run
```

//logic gates
fn and_gate( a:u32, b:u32)->u32{
    a&b
}
fn or_gate( a:u32, b:u32)->u32{
    a|b
}
fn xor_gate( a:u32, b:u32)->u32{
    a^b
}
// fn not_gate( a:u32, 
// )->u32{
//     !a&1
// }
//adders
fn half_adder(a: u32, b: u32) -> (u32, u32){
    let sum = xor_gate(a, b);
    let carry = and_gate(a, b);
     (sum, carry)   
}
fn full_adder(a: u32, b: u32, carry_in: u32) -> (u32, u32) {
   let (sum1, carry1) = half_adder(a, b);
    let (sum, carry2) = half_adder(sum1, carry_in);
    let carry_out = or_gate(carry1, carry2);
    (sum, carry_out)
 }
fn multi_bit_addition(mut a:u32,mut b:u32)->u32 {
    let mut carry = 0;
    let mut sum = 0;
    let mut bit_position = 0;
    
    while a > 0 || b > 0 || carry > 0 {
        let a_bit = a & 1;
        let b_bit = b & 1;

        let (sum_bit, carry_out) = full_adder(a_bit, b_bit, carry);
        
        sum |= sum_bit << bit_position;
        carry = carry_out;
        
        a >>= 1;
        b >>= 1;
        bit_position += 1;
    }
    
    sum
}
fn binary_subtraction(mut a: u32,  mut b: u32) -> u32 {
    let mut borrow = 0;
    let mut difference = 0;
    let mut bit_position = 0;

    while a > 0 || b > 0 {
        let a_bit = (a & 1) as i32;
        let b_bit = (b & 1) as i32;

        let diff_bit = a_bit - b_bit - borrow;

        if diff_bit >= 0 {
            difference |= (diff_bit as u32) << bit_position;
            borrow = 0;
        } else {
            difference |= 1 << bit_position;
            borrow = 1;
        }

        a >>= 1;
        b >>= 1;
        bit_position += 1;
    }

    difference
}

fn binary_multiplication(mut a: u32,mut b: u32) -> u32 {
    let mut product = 0;
    let mut bit_position = 0;
    
    while b > 0 {
        if b & 1 == 1 {
            product += a << bit_position;
        }
        b >>= 1;
        bit_position += 1;
    }
    
    product
}
fn binary_division(mut a: u32, mut b: u32) -> (u32, u32) {
    if b == 0 {
        panic!("Division by zero");
    }

    let mut quotient = 0;
    let mut remainder = a;

    let mut divisor_shift = b;
    let mut divisor_shifted = 1;

    while divisor_shifted <= a {
        divisor_shift <<= 1;
        divisor_shifted <<= 1;
    }

    while divisor_shifted > 0 {
        if remainder >= divisor_shift {
            remainder -= divisor_shift;
            quotient |= divisor_shifted >> 1;
        }
        divisor_shift >>= 1;
        divisor_shifted >>= 1;
    }

    (quotient, remainder)
}

use std::io;

fn main() {
    println!("Enter a  expression (e.g., 2+3, 4*5, 6-2, 8/4):");

    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let input = input.trim();


    let (num1, operator, num2) = parse_expression(input).expect("Invalid expression");

    
    let binary1 = decimal_to_binary_vec(num1 as u32);
    let binary2 = decimal_to_binary_vec(num2 as u32);

    let result = match operator {
        '+' => multi_bit_addition(num1 as u32, num2 as u32),
        '-' => binary_subtraction(num1 as u32, num2 as u32),
        '*' => binary_multiplication(num1 as u32, num2 as u32),
        '/' =>{ let (quotient, remainder) = binary_division(num1 as u32, num2 as u32);
        println!("Quotient: {}", quotient);
        println!("Remainder: {}", remainder);
        quotient // Return quotient if needed for further operations
    },
        _ => panic!("Unsupported operator"),

    };
    

    let binary_result = decimal_to_binary_vec(result as u32);

    println!("Answer is : {}", result);

}
// Function to parse the input expression
fn parse_expression(input: &str) -> Option<(i32, char, i32)> {
    let mut chars = input.chars();
    let mut num1 = String::new();
    let mut operator = None;
    let mut num2 = String::new();

    // Extract the first number
    while let Some(c) = chars.next() {
        if c.is_digit(10) {
            num1.push(c);
        } else {
            operator = Some(c);
            break;
        }
    }

    // Extract the second number
    while let Some(c) = chars.next() {
        if c.is_digit(10) {
            num2.push(c);
        }
    }

    if let (Ok(num1), Some(operator), Ok(num2)) = (num1.parse(), operator, num2.parse()) {
        Some((num1, operator, num2))
    } else {
        None
    }
}

fn decimal_to_binary_vec(mut num: u32) -> Vec<u8> {
    let mut binary = Vec::new();
    if num == 0 {
        binary.push(0);
    } else {
        while num > 0 {
            binary.push((num % 2) as u8); 
            num /= 2;                  
        }
        binary.reverse(); 
    }
    binary
}

// fn binary_to_decimal(binary: &[u8]) -> i32 {
//     let mut decimal = 0;
//     let mut base = 1; 
    
//     for &bit in binary.iter().rev() {
//         if bit == 1 {
//             decimal += base; 
//         }
//         base *= 2;

//     }
    
//     decimal
// }




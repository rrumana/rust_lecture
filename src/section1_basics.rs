//! Section 1: Basic Syntax and Constructs
//! =====================================
//! 
//! This section demonstrates fundamental Rust syntax including:
//! - Variables and mutability
//! - Functions and expressions
//! - Control flow (if, match, loops)
//! - Block expressions

#![allow(unused)]

/// Demo 1a: Hello World - The traditional first program
pub fn demo_hello_world() {
    println!("=== Demo 1a: Hello World ===");
    println!("Hello, Rust world!");
    println!();
}

/// Demo 1b: Variables and Mutability - Rust's default immutability
pub fn demo_variables_mutability() {
    println!("=== Demo 1b: Variables and Mutability ===");
    
    let x = 10;          // immutable by default
    let mut y = 5;       // mutable when declared with `mut`
    
    println!("Initial: x = {}, y = {}", x, y);
    
    // x = x + 1;        // This would cause a compile error!
    y = y + 1;           // This is OK because y is mutable
    
    println!("After modification: x = {}, y = {}", x, y);
    println!();
}

/// Demo 1c: Functions - Implicit returns and type annotations
pub fn demo_functions() {
    println!("=== Demo 1c: Functions ===");
    
    // Function with implicit return (no semicolon on last expression)
    fn add(a: i32, b: i32) -> i32 { 
        a + b  // No semicolon = return value
    }
    
    let result = add(5, 3);
    println!("add(5, 3) = {}", result);
    println!();
}

/// Demo 1d: If Expressions - if as an expression that returns values
pub fn demo_if_expressions() {
    println!("=== Demo 1d: If Expressions ===");
    
    let n = 42;
    
    // if-expression returns a value
    let parity = if n % 2 == 0 { "even" } else { "odd" };
    println!("{} is {}", n, parity);
    
    // More complex example
    let description = if n > 100 {
        "large"
    } else if n > 10 {
        "medium"
    } else {
        "small"
    };
    println!("{} is a {} number", n, description);
    println!();
}

/// Demo 1e: Match Expressions - Rust's powerful pattern matching
pub fn demo_match_expressions() {
    println!("=== Demo 1e: Match Expressions ===");
    
    fn describe_number(n: i32) {
        match n {
            1 => println!("one"),
            2 | 3 => println!("two or three"),           // Multiple patterns
            4..=10 => println!("between 4 and 10"),      // Range patterns
            x if x < 0 => println!("negative: {}", x),   // Guard conditions
            _ => println!("something else: {}", n),      // Catch-all
        }
    }
    
    let test_numbers = [1, 3, 7, -5, 42];
    for num in test_numbers {
        print!("{} → ", num);
        describe_number(num);
    }
    println!();
}

/// Demo 1f: For Loops - Iterating over ranges and collections
pub fn demo_for_loops() {
    println!("=== Demo 1f: For Loops ===");
    
    print!("Range 0..5: ");
    for x in 0..5 { 
        print!("{} ", x); 
    }
    println!();
    
    print!("Inclusive range 0..=5: ");
    for x in 0..=5 { 
        print!("{} ", x); 
    }
    println!();
    
    let fruits = ["apple", "banana", "cherry"];
    print!("Fruits: ");
    for fruit in fruits.iter() {
        print!("{} ", fruit);
    }
    println!();
    println!();
}

/// Demo 1g: While Loops - Conditional iteration
pub fn demo_while_loops() {
    println!("=== Demo 1g: While Loops ===");
    
    let mut i = 0;
    print!("Countdown: ");
    while i < 5 {
        print!("{} ", i);
        i += 1;
    }
    println!();
    println!();
}

/// Demo 1h: Block Expressions - Blocks that return values
pub fn demo_block_expressions() {
    println!("=== Demo 1h: Block Expressions ===");
    
    let result = {
        let a = 2;
        let b = 3;
        println!("Computing {} * {}", a, b);
        a * b   // last expression (no semicolon) is returned
    };
    
    println!("Block result: {}", result);
    
    // Blocks can be used anywhere expressions are expected
    let message = {
        if result > 5 {
            "large result"
        } else {
            "small result"
        }
    };
    
    println!("Message: {}", message);
    println!();
}

/// Run all demos in sequence
pub fn run_all_demos() {
    println!("🦀 RUST LECTURE - SECTION 1: BASIC SYNTAX AND CONSTRUCTS 🦀");
    println!("============================================================");
    println!();
    
    demo_hello_world();
    demo_variables_mutability();
    demo_functions();
    demo_if_expressions();
    demo_match_expressions();
    demo_for_loops();
    demo_while_loops();
    demo_block_expressions();
    
    println!("✅ Section 1 complete!");
}
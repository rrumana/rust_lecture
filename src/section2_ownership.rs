//! Section 2: Ownership and Move Semantics
//! =======================================
//! 
//! This section demonstrates Rust's unique ownership system:
//! - Ownership rules and scope
//! - Move semantics vs Copy semantics
//! - Function ownership transfer
//! - Memory safety without garbage collection

#![allow(unused)]

/// Demo 2a: Ownership and Scope - Variables are dropped when they go out of scope
pub fn demo_ownership_scope() {
    println!("=== Demo 2a: Ownership and Scope ===");
    
    {
        let s = String::from("hello");
        println!("s is valid here: {}", s);
        // s is automatically dropped when this scope ends
    }
    // s is no longer valid here - would cause compile error if used
    
    println!("String was automatically cleaned up when scope ended");
    println!();
}

/// Demo 2b: Move Semantics - Only one owner at a time for heap data
pub fn demo_move_semantics() {
    println!("=== Demo 2b: Move Semantics ===");
    
    let s1 = String::from("rust");
    println!("s1 = {}", s1);
    
    let s2 = s1;          // s1 is MOVED to s2, s1 is no longer valid
    println!("s2 = {}", s2);
    
    // println!("{}", s1); // This would cause a compile error!
    println!("s1 is no longer valid after the move");
    
    // But we can clone if we want both to be valid
    let s3 = s2.clone();
    println!("s2 = {}, s3 = {} (both valid after clone)", s2, s3);
    println!();
}

/// Demo 2c: Copy Types - Some types implement Copy trait for automatic copying
pub fn demo_copy_types() {
    println!("=== Demo 2c: Copy Types ===");
    
    // Primitive types implement Copy trait
    let x: i32 = 5;
    let y = x;            // x is copied, not moved
    println!("x = {}, y = {} (both still valid)", x, y);
    
    // Tuples of Copy types are also Copy
    let point1 = (3, 4);
    let point2 = point1;  // Copied, not moved
    println!("point1 = {:?}, point2 = {:?}", point1, point2);
    
    // Arrays of Copy types are Copy (if size is reasonable)
    let arr1 = [1, 2, 3];
    let arr2 = arr1;      // Copied
    println!("arr1 = {:?}, arr2 = {:?}", arr1, arr2);
    println!();
}

/// Demo 2d: Function Ownership Transfer - Functions can take ownership
pub fn demo_function_ownership() {
    println!("=== Demo 2d: Function Ownership Transfer ===");
    
    // Function that takes ownership
    fn consume_string(s: String) {
        println!("Function consumed: {}", s);
        // s is dropped when function ends
    }
    
    // Function that takes ownership and returns it back
    fn take_and_give_back(s: String) -> String {
        println!("Function processing: {}", s);
        s  // Return ownership back to caller
    }
    
    let s1 = String::from("hello");
    println!("Before function call: s1 = {}", s1);
    
    consume_string(s1);
    // s1 is no longer valid here!
    println!("s1 is no longer valid after consume_string");
    
    let s2 = String::from("world");
    let s3 = take_and_give_back(s2);
    // s2 is invalid, but s3 now owns the string
    println!("s3 = {} (ownership returned)", s3);
    println!();
}

/// Demo 2e: Ownership with Collections - Demonstrating moves in collections
pub fn demo_collection_ownership() {
    println!("=== Demo 2e: Ownership with Collections ===");
    
    let mut vec = Vec::new();
    
    let s1 = String::from("first");
    let s2 = String::from("second");
    
    println!("Before pushing: s1 = {}, s2 = {}", s1, s2);
    
    vec.push(s1);  // s1 is moved into the vector
    vec.push(s2);  // s2 is moved into the vector
    
    // s1 and s2 are no longer valid here
    println!("Strings moved into vector: {:?}", vec);
    
    // We can get ownership back by removing from vector
    if let Some(retrieved) = vec.pop() {
        println!("Retrieved from vector: {}", retrieved);
    }
    
    println!("Vector now: {:?}", vec);
    println!();
}

/// Demo 2f: Common Ownership Patterns - Practical examples
pub fn demo_ownership_patterns() {
    println!("=== Demo 2f: Common Ownership Patterns ===");
    
    // Pattern 1: Creating and returning owned data
    fn create_greeting(name: &str) -> String {
        format!("Hello, {}!", name)
    }
    
    let greeting = create_greeting("Alice");
    println!("Created: {}", greeting);
    
    // Pattern 2: Processing and transforming owned data
    fn make_uppercase(mut s: String) -> String {
        s = s.to_uppercase();
        s
    }
    
    let original = String::from("rust is awesome");
    let uppercase = make_uppercase(original);
    // original is no longer valid
    println!("Transformed: {}", uppercase);
    
    // Pattern 3: Conditional ownership transfer
    fn maybe_take_string(take: bool, s: String) -> Option<String> {
        if take {
            Some(s)
        } else {
            None
        }
    }
    
    let test_string = String::from("test");
    match maybe_take_string(true, test_string) {
        Some(s) => println!("Got back: {}", s),
        None => println!("String was consumed"),
    }
    println!();
}

/// Run all demos in sequence
pub fn run_all_demos() {
    println!("🦀 RUST LECTURE - SECTION 2: OWNERSHIP AND MOVE SEMANTICS 🦀");
    println!("==============================================================");
    println!();
    
    demo_ownership_scope();
    demo_move_semantics();
    demo_copy_types();
    demo_function_ownership();
    demo_collection_ownership();
    demo_ownership_patterns();
    
    println!("✅ Section 2 complete!");
    println!("💡 Key takeaway: Rust's ownership system prevents memory leaks and data races at compile time!");
}
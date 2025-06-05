//! Section 3: Borrowing, References, and Lifetimes
//! ===============================================
//! 
//! This section demonstrates Rust's borrowing system:
//! - Immutable and mutable references
//! - Borrowing rules and the borrow checker
//! - Lifetime annotations and lifetime elision
//! - Common borrowing patterns

#![allow(unused)]

/// Demo 3a: Immutable Borrowing - Reading data without taking ownership
pub fn demo_immutable_borrowing() {
    println!("=== Demo 3a: Immutable Borrowing ===");
    
    fn calculate_length(s: &String) -> usize {
        s.len()  // We can read the string but not modify it
    }
    
    let s1 = String::from("hello world");
    let length = calculate_length(&s1);  // Borrow s1
    
    println!("String: '{}' has length: {}", s1, length);
    // s1 is still valid because we only borrowed it!
    println!("s1 is still valid: {}", s1);
    
    // Multiple immutable borrows are allowed
    let r1 = &s1;
    let r2 = &s1;
    let r3 = &s1;
    println!("Multiple immutable borrows: {}, {}, {}", r1, r2, r3);
    println!();
}

/// Demo 3b: Mutable Borrowing - Modifying data through references
pub fn demo_mutable_borrowing() {
    println!("=== Demo 3b: Mutable Borrowing ===");
    
    fn append_exclamation(s: &mut String) {
        s.push_str("!");
    }
    
    let mut s = String::from("hello");
    println!("Before: {}", s);
    
    append_exclamation(&mut s);  // Mutable borrow
    println!("After: {}", s);
    
    // Direct mutable borrowing
    let mut x = 42;
    println!("x before: {}", x);
    
    {
        let mr = &mut x;  // Mutable reference
        *mr += 10;        // Dereference and modify
        println!("Modified through reference: {}", *mr);
    }  // mr goes out of scope here
    
    println!("x after: {}", x);
    println!();
}

/// Demo 3c: Borrowing Rules - The borrow checker in action
pub fn demo_borrowing_rules() {
    println!("=== Demo 3c: Borrowing Rules ===");
    
    let mut s = String::from("hello");
    
    // Rule 1: Multiple immutable borrows are OK
    let r1 = &s;
    let r2 = &s;
    println!("Multiple immutable borrows: {} and {}", r1, r2);
    
    // Rule 2: Only one mutable borrow at a time
    {
        let r3 = &mut s;
        r3.push_str(" world");
        println!("Mutable borrow: {}", r3);
        // Can't have other borrows while r3 is active
    }  // r3 goes out of scope
    
    // Rule 3: Can't mix mutable and immutable borrows
    let r4 = &s;  // OK, no mutable borrows active
    println!("Back to immutable: {}", r4);
    
    // Demonstrating scope-based borrowing
    let mut vec = vec![1, 2, 3];
    let first = &vec[0];  // Immutable borrow
    println!("First element: {}", first);
    // first is no longer used after this point
    
    vec.push(4);  // OK, no active borrows
    println!("Vector after push: {:?}", vec);
    println!();
}

/// Demo 3d: Lifetime Annotations - Explicit lifetime management
pub fn demo_lifetimes() {
    println!("=== Demo 3d: Lifetime Annotations ===");
    
    // Function with explicit lifetime annotation
    fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
        if x.len() > y.len() {
            x
        } else {
            y
        }
    }
    
    let string1 = String::from("long string is long");
    let string2 = "xyz";
    
    let result = longest(&string1, string2);
    println!("The longest string is: '{}'", result);
    
    // Lifetime with structs
    #[derive(Debug)]
    struct ImportantExcerpt<'a> {
        part: &'a str,
    }
    
    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.').next().expect("Could not find a '.'");
    let excerpt = ImportantExcerpt {
        part: first_sentence,
    };
    println!("Excerpt: {:?}", excerpt);
    println!();
}

/// Demo 3e: Lifetime Elision - When you don't need explicit lifetimes
pub fn demo_lifetime_elision() {
    println!("=== Demo 3e: Lifetime Elision ===");
    
    // These functions don't need explicit lifetime annotations
    // due to lifetime elision rules
    
    fn first_word(s: &str) -> &str {
        let bytes = s.as_bytes();
        for (i, &item) in bytes.iter().enumerate() {
            if item == b' ' {
                return &s[0..i];
            }
        }
        &s[..]
    }
    
    fn get_first_char(s: &str) -> &str {
        &s[0..1]
    }
    
    let sentence = "hello world rust";
    let first = first_word(sentence);
    let first_char = get_first_char(sentence);
    
    println!("Sentence: '{}'", sentence);
    println!("First word: '{}'", first);
    println!("First char: '{}'", first_char);
    println!();
}

/// Demo 3f: Common Reference Patterns - Practical borrowing scenarios
pub fn demo_reference_patterns() {
    println!("=== Demo 3f: Common Reference Patterns ===");
    
    // Pattern 1: Slice borrowing
    fn print_slice(slice: &[i32]) {
        println!("Slice: {:?}", slice);
    }
    
    let arr = [1, 2, 3, 4, 5];
    let vec = vec![10, 20, 30];
    
    print_slice(&arr);        // Array slice
    print_slice(&vec);        // Vector slice
    print_slice(&vec[1..3]);  // Partial slice
    
    // Pattern 2: String slices
    fn process_name(name: &str) -> String {
        format!("Hello, {}!", name)
    }
    
    let owned_string = String::from("Alice");
    let string_literal = "Bob";
    
    println!("{}", process_name(&owned_string));  // String reference
    println!("{}", process_name(string_literal)); // String literal
    
    // Pattern 3: Optional references
    fn maybe_print(opt_ref: Option<&str>) {
        match opt_ref {
            Some(s) => println!("Got: {}", s),
            None => println!("Got nothing"),
        }
    }
    
    let some_string = String::from("test");
    maybe_print(Some(&some_string));
    maybe_print(None);
    println!();
}

/// Demo 3g: Dangling References - What the borrow checker prevents
pub fn demo_dangling_prevention() {
    println!("=== Demo 3g: Dangling Reference Prevention ===");
    
    // This function would create a dangling reference (won't compile)
    // fn dangle() -> &String {
    //     let s = String::from("hello");
    //     &s  // ERROR: s is dropped but we're returning a reference to it
    // }
    
    // Correct way: return owned data
    fn no_dangle() -> String {
        let s = String::from("hello");
        s  // Return ownership, not a reference
    }
    
    let result = no_dangle();
    println!("Safe result: {}", result);
    
    // Another example of what borrow checker prevents
    // let r;
    // {
    //     let x = 5;
    //     r = &x;  // ERROR: x doesn't live long enough
    // }
    // println!("{}", r);  // x is already dropped!
    
    println!("Borrow checker prevents dangling references at compile time!");
    println!();
}

/// Run all demos in sequence
pub fn run_all_demos() {
    println!("🦀 RUST LECTURE - SECTION 3: BORROWING, REFERENCES, AND LIFETIMES 🦀");
    println!("======================================================================");
    println!();
    
    demo_immutable_borrowing();
    demo_mutable_borrowing();
    demo_borrowing_rules();
    demo_lifetimes();
    demo_lifetime_elision();
    demo_reference_patterns();
    demo_dangling_prevention();
    
    println!("✅ Section 3 complete!");
    println!("💡 Key takeaway: Borrowing allows safe access to data without ownership transfer!");
}
//! Section 6: Idiomatic Patterns & Utilities
//! =========================================
//! 
//! This section demonstrates idiomatic Rust patterns:
//! - Iterator patterns and functional programming
//! - Error handling with ? operator
//! - Variable shadowing and type transformations
//! - Common utility patterns
//! - Memory-efficient programming techniques

#![allow(unused)]

/// Demo 6a: Iterator Patterns - Functional programming in Rust
pub fn demo_iterator_patterns() {
    println!("=== Demo 6a: Iterator Patterns ===");
    
    let numbers = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    
    // Basic iteration
    println!("Original numbers: {:?}", numbers);
    
    // Map - transform each element
    let doubled: Vec<i32> = numbers.iter().map(|x| x * 2).collect();
    println!("Doubled: {:?}", doubled);
    
    // Filter - keep elements that match condition
    let evens: Vec<&i32> = numbers.iter().filter(|&&x| x % 2 == 0).collect();
    println!("Even numbers: {:?}", evens);
    
    // Chain operations
    let processed: Vec<i32> = numbers
        .iter()
        .filter(|&&x| x > 3)        // Keep numbers > 3
        .map(|x| x * x)             // Square them
        .collect();
    println!("Filtered and squared: {:?}", processed);
    
    // Reduce operations
    let sum: i32 = numbers.iter().sum();
    let product: i32 = numbers.iter().product();
    let max = numbers.iter().max();
    
    println!("Sum: {}, Product: {}, Max: {:?}", sum, product, max);
    
    // Find operations
    let first_even = numbers.iter().find(|&&x| x % 2 == 0);
    let position = numbers.iter().position(|&x| x == 5);
    
    println!("First even: {:?}, Position of 5: {:?}", first_even, position);
    
    // Any/All
    let has_even = numbers.iter().any(|&x| x % 2 == 0);
    let all_positive = numbers.iter().all(|&x| x > 0);
    
    println!("Has even: {}, All positive: {}", has_even, all_positive);
    
    // Enumerate - get index with value
    println!("Enumerated:");
    for (index, value) in numbers.iter().enumerate() {
        if index < 3 {
            println!("  [{}]: {}", index, value);
        }
    }
    
    // Zip - combine two iterators
    let letters = vec!['a', 'b', 'c', 'd'];
    let zipped: Vec<(i32, char)> = numbers.iter().take(4).cloned().zip(letters).collect();
    println!("Zipped: {:?}", zipped);
    println!();
}

/// Demo 6b: Advanced Iterator Techniques
pub fn demo_advanced_iterators() {
    println!("=== Demo 6b: Advanced Iterator Techniques ===");
    
    // Custom iterator
    struct Counter {
        current: usize,
        max: usize,
    }
    
    impl Counter {
        fn new(max: usize) -> Counter {
            Counter { current: 0, max }
        }
    }
    
    impl Iterator for Counter {
        type Item = usize;
        
        fn next(&mut self) -> Option<Self::Item> {
            if self.current < self.max {
                let current = self.current;
                self.current += 1;
                Some(current)
            } else {
                None
            }
        }
    }
    
    let counter_values: Vec<usize> = Counter::new(5).collect();
    println!("Custom counter: {:?}", counter_values);
    
    // Lazy evaluation demonstration
    let expensive_computation = (1..1000000)
        .map(|x| {
            // This closure is only called when needed
            x * x
        })
        .filter(|&x| x % 1000 == 0)
        .take(5);  // Only compute first 5 matches
    
    let results: Vec<i32> = expensive_computation.collect();
    println!("Lazy computation results: {:?}", results);
    
    // Flat map - flatten nested structures
    let nested = vec![vec![1, 2], vec![3, 4, 5], vec![6]];
    let flattened: Vec<i32> = nested.into_iter().flat_map(|v| v).collect();
    println!("Flattened: {:?}", flattened);
    
    // Partition - split into two collections
    let numbers = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    let (evens, odds): (Vec<i32>, Vec<i32>) = numbers.into_iter().partition(|&x| x % 2 == 0);
    println!("Evens: {:?}, Odds: {:?}", evens, odds);
    
    // Group by (using itertools would be better, but showing manual approach)
    let words = vec!["apple", "banana", "apricot", "blueberry", "avocado"];
    let mut grouped = std::collections::HashMap::new();
    for word in words {
        let first_char = word.chars().next().unwrap();
        grouped.entry(first_char).or_insert(Vec::new()).push(word);
    }
    println!("Grouped by first letter: {:?}", grouped);
    println!();
}

/// Demo 6c: Error Handling Patterns
pub fn demo_error_handling_patterns() {
    println!("=== Demo 6c: Error Handling Patterns ===");
    
    use std::fs::File;
    use std::io::{self, Read};
    
    // Pattern 1: Early return with ?
    fn read_file_content(filename: &str) -> io::Result<String> {
        let mut file = File::open(filename)?;
        let mut content = String::new();
        file.read_to_string(&mut content)?;
        Ok(content)
    }
    
    // Pattern 2: Multiple error types with custom enum
    #[derive(Debug)]
    enum AppError {
        Io(io::Error),
        Parse(std::num::ParseIntError),
        Custom(String),
    }
    
    impl From<io::Error> for AppError {
        fn from(error: io::Error) -> Self {
            AppError::Io(error)
        }
    }
    
    impl From<std::num::ParseIntError> for AppError {
        fn from(error: std::num::ParseIntError) -> Self {
            AppError::Parse(error)
        }
    }
    
    fn process_config_file(filename: &str) -> Result<i32, AppError> {
        let content = std::fs::read_to_string(filename)?;
        
        if content.trim().is_empty() {
            return Err(AppError::Custom("File is empty".to_string()));
        }
        
        let number: i32 = content.trim().parse()?;
        
        if number < 0 {
            return Err(AppError::Custom("Number must be positive".to_string()));
        }
        
        Ok(number)
    }
    
    // Pattern 3: Result combinators
    fn demonstrate_result_combinators() {
        let result: Result<i32, &str> = Ok(42);
        
        // map transforms the Ok value
        let doubled = result.map(|x| x * 2);
        println!("Doubled result: {:?}", doubled);
        
        // map_err transforms the Err value
        let with_custom_error = result.map_err(|e| format!("Custom error: {}", e));
        println!("Custom error result: {:?}", with_custom_error);
        
        // and_then chains operations that might fail
        let chained = result.and_then(|x| {
            if x > 40 {
                Ok(x + 10)
            } else {
                Err("Too small")
            }
        });
        println!("Chained result: {:?}", chained);
        
        // unwrap_or provides default on error
        let error_result: Result<i32, &str> = Err("failed");
        let with_default = error_result.unwrap_or(0);
        println!("With default: {}", with_default);
    }
    
    demonstrate_result_combinators();
    
    // Test our functions
    match read_file_content("nonexistent.txt") {
        Ok(content) => println!("File content: {}", content),
        Err(e) => println!("Failed to read file: {}", e),
    }
    
    match process_config_file("nonexistent.txt") {
        Ok(number) => println!("Config number: {}", number),
        Err(e) => println!("Config error: {:?}", e),
    }
    println!();
}

/// Demo 6d: Variable Shadowing and Type Transformations
pub fn demo_shadowing_patterns() {
    println!("=== Demo 6d: Variable Shadowing and Type Transformations ===");
    
    // Basic shadowing for type transformation
    let input = "42";
    println!("Original input (str): {}", input);
    
    let input: i32 = input.parse().expect("Failed to parse");
    println!("Parsed input (i32): {}", input);
    
    let input = input * 2;
    println!("Doubled input (i32): {}", input);
    
    let input = format!("Result: {}", input);
    println!("Formatted input (String): {}", input);
    
    // Shadowing in different scopes
    let x = 5;
    println!("Outer x: {}", x);
    
    {
        let x = x * 2;  // Shadow outer x
        println!("Inner x: {}", x);
        
        let x = "hello";  // Shadow with different type
        println!("Inner x (string): {}", x);
    }
    
    println!("Outer x (unchanged): {}", x);
    
    // Practical shadowing example
    fn process_user_input(input: &str) -> Result<i32, String> {
        let input = input.trim();  // Shadow with trimmed version
        
        if input.is_empty() {
            return Err("Input is empty".to_string());
        }
        
        let input: i32 = input.parse()
            .map_err(|_| "Invalid number format".to_string())?;
        
        let input = if input < 0 { 0 } else { input };  // Clamp to positive
        
        Ok(input)
    }
    
    let test_inputs = ["  42  ", "-10", "abc", ""];
    for test in test_inputs {
        match process_user_input(test) {
            Ok(result) => println!("'{}' -> {}", test, result),
            Err(e) => println!("'{}' -> Error: {}", test, e),
        }
    }
    println!();
}

/// Demo 6e: Memory-Efficient Patterns
pub fn demo_memory_patterns() {
    println!("=== Demo 6e: Memory-Efficient Patterns ===");
    
    // Pattern 1: Using string slices instead of owned strings
    fn process_text_efficient(text: &str) -> Vec<&str> {
        text.split_whitespace()
            .filter(|word| word.len() > 3)
            .collect()
    }
    
    fn process_text_inefficient(text: &str) -> Vec<String> {
        text.split_whitespace()
            .filter(|word| word.len() > 3)
            .map(|word| word.to_string())  // Unnecessary allocation
            .collect()
    }
    
    let text = "The quick brown fox jumps over the lazy dog";
    let efficient_result = process_text_efficient(text);
    let inefficient_result = process_text_inefficient(text);
    
    println!("Efficient (slices): {:?}", efficient_result);
    println!("Inefficient (owned): {:?}", inefficient_result);
    
    // Pattern 2: Iterator chains instead of intermediate collections
    let numbers = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    
    // Inefficient: creates intermediate vectors
    let _inefficient = {
        let doubled: Vec<i32> = numbers.iter().map(|x| x * 2).collect();
        let filtered: Vec<i32> = doubled.into_iter().filter(|&x| x > 10).collect();
        let summed: i32 = filtered.iter().sum();
        summed
    };
    
    // Efficient: single iterator chain
    let efficient: i32 = numbers
        .iter()
        .map(|x| x * 2)
        .filter(|&x| x > 10)
        .sum();
    
    println!("Efficient sum: {}", efficient);
    
    // Pattern 3: Cow (Clone on Write) for conditional ownership
    use std::borrow::Cow;
    
    fn process_maybe_modify(input: &str, should_modify: bool) -> Cow<str> {
        if should_modify {
            Cow::Owned(input.to_uppercase())  // Allocate new string
        } else {
            Cow::Borrowed(input)              // Use original string
        }
    }
    
    let original = "hello world";
    let borrowed_result = process_maybe_modify(original, false);
    let owned_result = process_maybe_modify(original, true);
    
    println!("Original: {}", original);
    println!("Borrowed: {}", borrowed_result);
    println!("Owned: {}", owned_result);
    
    // Pattern 4: Using capacity hints
    fn build_string_efficient(words: &[&str]) -> String {
        let total_len: usize = words.iter().map(|s| s.len()).sum();
        let mut result = String::with_capacity(total_len + words.len() - 1);
        
        for (i, word) in words.iter().enumerate() {
            if i > 0 {
                result.push(' ');
            }
            result.push_str(word);
        }
        
        result
    }
    
    let words = ["efficient", "memory", "usage", "in", "rust"];
    let sentence = build_string_efficient(&words);
    println!("Built sentence: {}", sentence);
    println!();
}

/// Demo 6f: Common Utility Patterns
pub fn demo_utility_patterns() {
    println!("=== Demo 6f: Common Utility Patterns ===");
    
    // Pattern 1: Builder pattern
    #[derive(Debug)]
    struct Config {
        host: String,
        port: u16,
        timeout: u64,
        retries: u32,
    }
    
    impl Config {
        fn new() -> ConfigBuilder {
            ConfigBuilder::default()
        }
    }
    
    #[derive(Default)]
    struct ConfigBuilder {
        host: Option<String>,
        port: Option<u16>,
        timeout: Option<u64>,
        retries: Option<u32>,
    }
    
    impl ConfigBuilder {
        fn host(mut self, host: &str) -> Self {
            self.host = Some(host.to_string());
            self
        }
        
        fn port(mut self, port: u16) -> Self {
            self.port = Some(port);
            self
        }
        
        fn timeout(mut self, timeout: u64) -> Self {
            self.timeout = Some(timeout);
            self
        }
        
        fn retries(mut self, retries: u32) -> Self {
            self.retries = Some(retries);
            self
        }
        
        fn build(self) -> Config {
            Config {
                host: self.host.unwrap_or_else(|| "localhost".to_string()),
                port: self.port.unwrap_or(8080),
                timeout: self.timeout.unwrap_or(30),
                retries: self.retries.unwrap_or(3),
            }
        }
    }
    
    let config = Config::new()
        .host("example.com")
        .port(443)
        .timeout(60)
        .build();
    
    println!("Config: {:?}", config);
    
    // Pattern 2: Newtype pattern for type safety
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    struct UserId(u64);
    
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    struct ProductId(u64);
    
    impl UserId {
        fn new(id: u64) -> Self {
            UserId(id)
        }
        
        fn value(self) -> u64 {
            self.0
        }
    }
    
    impl ProductId {
        fn new(id: u64) -> Self {
            ProductId(id)
        }
        
        fn value(self) -> u64 {
            self.0
        }
    }
    
    fn get_user_orders(user_id: UserId) -> Vec<ProductId> {
        println!("Getting orders for user {}", user_id.value());
        vec![ProductId::new(101), ProductId::new(102)]
    }
    
    let user = UserId::new(42);
    let orders = get_user_orders(user);
    println!("User orders: {:?}", orders);
    
    // This would be a compile error - type safety!
    // get_user_orders(ProductId::new(123));
    
    // Pattern 3: Extension traits
    trait StringExtensions {
        fn is_palindrome(&self) -> bool;
        fn word_count(&self) -> usize;
    }
    
    impl StringExtensions for str {
        fn is_palindrome(&self) -> bool {
            let cleaned: String = self.chars()
                .filter(|c| c.is_alphanumeric())
                .map(|c| c.to_lowercase().next().unwrap())
                .collect();
            
            cleaned == cleaned.chars().rev().collect::<String>()
        }
        
        fn word_count(&self) -> usize {
            self.split_whitespace().count()
        }
    }
    
    let text = "A man a plan a canal Panama";
    println!("'{}' is palindrome: {}", text, text.is_palindrome());
    println!("Word count: {}", text.word_count());
    println!();
}

/// Run all demos in sequence
pub fn run_all_demos() {
    println!("🦀 RUST LECTURE - SECTION 6: IDIOMATIC PATTERNS & UTILITIES 🦀");
    println!("================================================================");
    println!();
    
    demo_iterator_patterns();
    demo_advanced_iterators();
    demo_error_handling_patterns();
    demo_shadowing_patterns();
    demo_memory_patterns();
    demo_utility_patterns();
    
    println!("✅ Section 6 complete!");
    println!("💡 Key takeaway: Idiomatic Rust emphasizes zero-cost abstractions and memory efficiency!");
}

/// Get list of available demos for enhanced navigation
pub fn get_demo_list() -> Vec<&'static str> {
    vec![
        "iterators",
        "advanced_iterators",
        "errors",
        "shadowing",
        "memory",
        "utilities",
    ]
}
//! Section 5: Enums, Pattern Matching, Option & Result
//! ===================================================
//! 
//! This section demonstrates Rust's powerful enum system:
//! - Defining and using enums
//! - Pattern matching with match
//! - Option<T> for handling null values
//! - Result<T, E> for error handling
//! - Advanced pattern matching techniques

#![allow(unused)]

/// Demo 5a: Basic Enums - Defining types with multiple variants
pub fn demo_basic_enums() {
    println!("=== Demo 5a: Basic Enums ===");
    
    #[derive(Debug)]
    enum Direction {
        North,
        South,
        East,
        West,
    }
    
    fn move_player(direction: Direction) {
        match direction {
            Direction::North => println!("Moving north"),
            Direction::South => println!("Moving south"),
            Direction::East => println!("Moving east"),
            Direction::West => println!("Moving west"),
        }
    }
    
    let directions = [
        Direction::North,
        Direction::East,
        Direction::South,
        Direction::West,
    ];
    
    for dir in directions {
        println!("Direction: {:?}", dir);
        move_player(dir);
    }
    println!();
}

/// Demo 5b: Enums with Data - Variants can hold different types of data
pub fn demo_enums_with_data() {
    println!("=== Demo 5b: Enums with Data ===");
    
    #[derive(Debug)]
    enum Message {
        Quit,                       // No data
        Move { x: i32, y: i32 },   // Named fields
        Write(String),              // Single value
        ChangeColor(i32, i32, i32), // Tuple
    }
    
    impl Message {
        fn process(&self) {
            match self {
                Message::Quit => println!("Quitting application"),
                Message::Move { x, y } => println!("Moving to ({}, {})", x, y),
                Message::Write(text) => println!("Writing: {}", text),
                Message::ChangeColor(r, g, b) => println!("Changing color to RGB({}, {}, {})", r, g, b),
            }
        }
    }
    
    let messages = vec![
        Message::Quit,
        Message::Move { x: 10, y: 20 },
        Message::Write("Hello, Rust!".to_string()),
        Message::ChangeColor(255, 0, 128),
    ];
    
    for msg in messages {
        println!("Message: {:?}", msg);
        msg.process();
        println!();
    }
}

/// Demo 5c: Option<T> - Handling the absence of values safely
pub fn demo_option_type() {
    println!("=== Demo 5c: Option<T> ===");
    
    fn find_word(text: &str, word: &str) -> Option<usize> {
        text.find(word)
    }
    
    fn divide(a: f64, b: f64) -> Option<f64> {
        if b != 0.0 {
            Some(a / b)
        } else {
            None
        }
    }
    
    let text = "Hello, Rust world!";
    let search_word = "Rust";
    
    match find_word(text, search_word) {
        Some(index) => println!("Found '{}' at index {}", search_word, index),
        None => println!("'{}' not found", search_word),
    }
    
    // Using if let for simpler pattern matching
    if let Some(index) = find_word(text, "world") {
        println!("Found 'world' at index {}", index);
    } else {
        println!("'world' not found");
    }
    
    // Option methods
    let some_number = Some(5);
    let no_number: Option<i32> = None;
    
    println!("some_number.is_some(): {}", some_number.is_some());
    println!("no_number.is_none(): {}", no_number.is_none());
    
    // unwrap_or provides default value
    println!("some_number.unwrap_or(0): {}", some_number.unwrap_or(0));
    println!("no_number.unwrap_or(0): {}", no_number.unwrap_or(0));
    
    // map transforms the value if present
    let doubled = some_number.map(|x| x * 2);
    println!("doubled: {:?}", doubled);
    
    // Division examples
    println!("10.0 / 2.0 = {:?}", divide(10.0, 2.0));
    println!("10.0 / 0.0 = {:?}", divide(10.0, 0.0));
    println!();
}

/// Demo 5d: Result<T, E> - Comprehensive error handling
pub fn demo_result_type() {
    println!("=== Demo 5d: Result<T, E> ===");
    
    #[derive(Debug)]
    enum MathError {
        DivisionByZero,
        NegativeSquareRoot,
    }
    
    fn safe_divide(a: f64, b: f64) -> Result<f64, MathError> {
        if b == 0.0 {
            Err(MathError::DivisionByZero)
        } else {
            Ok(a / b)
        }
    }
    
    fn safe_sqrt(x: f64) -> Result<f64, MathError> {
        if x < 0.0 {
            Err(MathError::NegativeSquareRoot)
        } else {
            Ok(x.sqrt())
        }
    }
    
    // Pattern matching with Result
    match safe_divide(10.0, 2.0) {
        Ok(result) => println!("10.0 / 2.0 = {}", result),
        Err(error) => println!("Error: {:?}", error),
    }
    
    match safe_divide(10.0, 0.0) {
        Ok(result) => println!("10.0 / 0.0 = {}", result),
        Err(error) => println!("Error: {:?}", error),
    }
    
    // Using if let with Result
    if let Ok(result) = safe_sqrt(16.0) {
        println!("sqrt(16.0) = {}", result);
    }
    
    if let Err(error) = safe_sqrt(-4.0) {
        println!("sqrt(-4.0) failed: {:?}", error);
    }
    
    // Result methods
    let good_result = safe_divide(20.0, 4.0);
    let bad_result = safe_divide(20.0, 0.0);
    
    println!("good_result.is_ok(): {}", good_result.is_ok());
    println!("bad_result.is_err(): {}", bad_result.is_err());
    
    // unwrap_or provides default on error
    println!("bad_result.unwrap_or(0.0): {}", bad_result.unwrap_or(0.0));
    
    // map transforms the Ok value
    let doubled_result = good_result.map(|x| x * 2.0);
    println!("doubled_result: {:?}", doubled_result);
    println!();
}

/// Demo 5e: Advanced Pattern Matching - Complex patterns and guards
pub fn demo_advanced_patterns() {
    println!("=== Demo 5e: Advanced Pattern Matching ===");
    
    #[derive(Debug, Clone, Copy)]
    struct Point {
        x: i32,
        y: i32,
    }
    
    fn describe_point(point: Point) -> &'static str {
        match point {
            Point { x: 0, y: 0 } => "at the origin",
            Point { x: 0, y: _ } => "on the Y axis",
            Point { x: _, y: 0 } => "on the X axis",
            Point { x, y } if x == y => "on the line y=x",
            Point { x, y } if x > 0 && y > 0 => "in quadrant I",
            Point { x, y } if x < 0 && y > 0 => "in quadrant II",
            Point { x, y } if x < 0 && y < 0 => "in quadrant III",
            Point { x, y } if x > 0 && y < 0 => "in quadrant IV",
            _ => "somewhere else",
        }
    }
    
    let points = vec![
        Point { x: 0, y: 0 },
        Point { x: 0, y: 5 },
        Point { x: 3, y: 0 },
        Point { x: 3, y: 3 },
        Point { x: 2, y: 5 },
        Point { x: -1, y: 3 },
        Point { x: -2, y: -4 },
        Point { x: 3, y: -2 },
    ];
    
    for point in points {
        println!("{:?} is {}", point, describe_point(point));
    }
    
    // Destructuring in let statements
    let point = Point { x: 10, y: 20 };
    let Point { x, y } = point;
    println!("Destructured: x = {}, y = {}", x, y);
    
    // Nested pattern matching
    let nested_option = Some(Some(42));
    match nested_option {
        Some(Some(value)) => println!("Nested value: {}", value),
        Some(None) => println!("Outer Some, inner None"),
        None => println!("Outer None"),
    }
    println!();
}

/// Demo 5f: Recursive Enums - Building complex data structures
pub fn demo_recursive_enums() {
    println!("=== Demo 5f: Recursive Enums ===");
    
    #[derive(Debug)]
    enum List<T> {
        Cons(T, Box<List<T>>),
        Nil,
    }
    
    impl<T> List<T> {
        fn new() -> Self {
            List::Nil
        }
        
        fn prepend(self, elem: T) -> Self {
            List::Cons(elem, Box::new(self))
        }
        
        fn len(&self) -> usize {
            match self {
                List::Nil => 0,
                List::Cons(_, tail) => 1 + tail.len(),
            }
        }
        
        fn stringify(&self) -> String 
        where 
            T: std::fmt::Display,
        {
            match self {
                List::Nil => "Nil".to_string(),
                List::Cons(head, tail) => format!("{}, {}", head, tail.stringify()),
            }
        }
    }
    
    let list = List::new()
        .prepend(1)
        .prepend(2)
        .prepend(3);
    
    println!("List: {}", list.stringify());
    println!("Length: {}", list.len());
    
    // Binary tree example
    #[derive(Debug)]
    enum BinaryTree<T> {
        Empty,
        Node {
            value: T,
            left: Box<BinaryTree<T>>,
            right: Box<BinaryTree<T>>,
        },
    }
    
    impl<T> BinaryTree<T> {
        fn new() -> Self {
            BinaryTree::Empty
        }
        
        fn leaf(value: T) -> Self {
            BinaryTree::Node {
                value,
                left: Box::new(BinaryTree::Empty),
                right: Box::new(BinaryTree::Empty),
            }
        }
        
        fn count_nodes(&self) -> usize {
            match self {
                BinaryTree::Empty => 0,
                BinaryTree::Node { left, right, .. } => {
                    1 + left.count_nodes() + right.count_nodes()
                }
            }
        }
    }
    
    let tree = BinaryTree::Node {
        value: 1,
        left: Box::new(BinaryTree::leaf(2)),
        right: Box::new(BinaryTree::Node {
            value: 3,
            left: Box::new(BinaryTree::leaf(4)),
            right: Box::new(BinaryTree::Empty),
        }),
    };
    
    println!("Tree nodes: {}", tree.count_nodes());
    println!();
}

/// Demo 5g: Error Propagation with ? operator
pub fn demo_error_propagation() {
    println!("=== Demo 5g: Error Propagation ===");
    
    use std::fs::File;
    use std::io::{self, Read};
    
    // Manual error handling
    fn read_username_manual() -> Result<String, io::Error> {
        let mut file = match File::open("username.txt") {
            Ok(file) => file,
            Err(e) => return Err(e),
        };
        
        let mut username = String::new();
        match file.read_to_string(&mut username) {
            Ok(_) => Ok(username),
            Err(e) => Err(e),
        }
    }
    
    // Using ? operator for cleaner code
    fn read_username_with_question_mark() -> Result<String, io::Error> {
        let mut file = File::open("username.txt")?;
        let mut username = String::new();
        file.read_to_string(&mut username)?;
        Ok(username)
    }
    
    // Even more concise
    fn read_username_concise() -> Result<String, io::Error> {
        std::fs::read_to_string("username.txt")
    }
    
    // Demonstrate the functions (they'll fail since file doesn't exist)
    match read_username_concise() {
        Ok(username) => println!("Username: {}", username),
        Err(error) => println!("Failed to read username: {}", error),
    }
    
    // Custom error types with ?
    #[derive(Debug)]
    enum CustomError {
        Io(io::Error),
        Parse(std::num::ParseIntError),
    }
    
    impl From<io::Error> for CustomError {
        fn from(error: io::Error) -> Self {
            CustomError::Io(error)
        }
    }
    
    impl From<std::num::ParseIntError> for CustomError {
        fn from(error: std::num::ParseIntError) -> Self {
            CustomError::Parse(error)
        }
    }
    
    fn read_and_parse_number() -> Result<i32, CustomError> {
        let content = std::fs::read_to_string("number.txt")?;  // io::Error -> CustomError
        let number: i32 = content.trim().parse()?;             // ParseIntError -> CustomError
        Ok(number)
    }
    
    match read_and_parse_number() {
        Ok(number) => println!("Number: {}", number),
        Err(error) => println!("Error: {:?}", error),
    }
    println!();
}

/// Run all demos in sequence
pub fn run_all_demos() {
    println!("🦀 RUST LECTURE - SECTION 5: ENUMS, PATTERN MATCHING, OPTION & RESULT 🦀");
    println!("==============================================================================");
    println!();
    
    demo_basic_enums();
    demo_enums_with_data();
    demo_option_type();
    demo_result_type();
    demo_advanced_patterns();
    demo_recursive_enums();
    demo_error_propagation();
    
    println!("✅ Section 5 complete!");
    println!("💡 Key takeaway: Enums and pattern matching provide safe, expressive error handling!");
}
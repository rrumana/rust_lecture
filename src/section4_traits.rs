//! Section 4: Trait System and Generics
//! ====================================
//! 
//! This section demonstrates Rust's powerful trait system:
//! - Defining and implementing traits
//! - Generic functions and types
//! - Trait bounds and where clauses
//! - Static vs dynamic dispatch
//! - Common standard library traits

#![allow(unused)]

/// Demo 4a: Basic Traits - Defining shared behavior
pub fn demo_basic_traits() {
    println!("=== Demo 4a: Basic Traits ===");
    
    // Define a trait
    trait Printable {
        fn print(&self);
        
        // Default implementation
        fn print_twice(&self) {
            self.print();
            self.print();
        }
    }
    
    // Implement trait for different types
    struct Point {
        x: i32,
        y: i32,
    }
    
    impl Printable for Point {
        fn print(&self) {
            println!("Point({}, {})", self.x, self.y);
        }
    }
    
    struct Circle {
        radius: f64,
    }
    
    impl Printable for Circle {
        fn print(&self) {
            println!("Circle with radius {}", self.radius);
        }
    }
    
    let point = Point { x: 3, y: 4 };
    let circle = Circle { radius: 5.0 };
    
    point.print();
    circle.print();
    
    println!("Using default implementation:");
    point.print_twice();
    println!();
}

/// Demo 4b: Generic Functions - Functions that work with multiple types
pub fn demo_generic_functions() {
    println!("=== Demo 4b: Generic Functions ===");
    
    // Generic function with trait bound
    fn print_anything<T: std::fmt::Display>(item: T) {
        println!("Item: {}", item);
    }
    
    // Multiple trait bounds
    fn print_and_debug<T: std::fmt::Display + std::fmt::Debug>(item: T) {
        println!("Display: {}", item);
        println!("Debug: {:?}", item);
    }
    
    // Using where clause for complex bounds
    fn complex_function<T, U>(t: T, u: U) 
    where 
        T: std::fmt::Display + Clone,
        U: std::fmt::Debug,
    {
        println!("T: {}", t);
        println!("U: {:?}", u);
        let _t_clone = t.clone();
    }
    
    print_anything(42);
    print_anything("hello");
    print_anything(3.14);
    
    println!();
    print_and_debug(100);
    
    println!();
    complex_function("test", vec![1, 2, 3]);
    println!();
}

/// Demo 4c: Trait Objects and Dynamic Dispatch
pub fn demo_trait_objects() {
    println!("=== Demo 4c: Trait Objects and Dynamic Dispatch ===");
    
    trait Drawable {
        fn draw(&self);
        fn area(&self) -> f64;
    }
    
    struct Rectangle {
        width: f64,
        height: f64,
    }
    
    impl Drawable for Rectangle {
        fn draw(&self) {
            println!("Drawing rectangle {}x{}", self.width, self.height);
        }
        
        fn area(&self) -> f64 {
            self.width * self.height
        }
    }
    
    struct Circle {
        radius: f64,
    }
    
    impl Drawable for Circle {
        fn draw(&self) {
            println!("Drawing circle with radius {}", self.radius);
        }
        
        fn area(&self) -> f64 {
            std::f64::consts::PI * self.radius * self.radius
        }
    }
    
    // Static dispatch - compiler knows exact type
    fn draw_rectangle(rect: &Rectangle) {
        rect.draw();
    }
    
    // Dynamic dispatch - runtime polymorphism
    fn draw_shape(shape: &dyn Drawable) {
        shape.draw();
        println!("Area: {:.2}", shape.area());
    }
    
    let rect = Rectangle { width: 10.0, height: 5.0 };
    let circle = Circle { radius: 3.0 };
    
    // Static dispatch
    draw_rectangle(&rect);
    
    // Dynamic dispatch
    draw_shape(&rect);
    draw_shape(&circle);
    
    // Collection of trait objects
    let shapes: Vec<Box<dyn Drawable>> = vec![
        Box::new(Rectangle { width: 2.0, height: 3.0 }),
        Box::new(Circle { radius: 1.0 }),
        Box::new(Rectangle { width: 4.0, height: 4.0 }),
    ];
    
    println!("\nDrawing all shapes:");
    for shape in shapes.iter() {
        shape.draw();
    }
    println!();
}

/// Demo 4d: Generic Structs and Implementations
pub fn demo_generic_structs() {
    println!("=== Demo 4d: Generic Structs ===");
    
    // Generic struct
    #[derive(Debug)]
    struct Pair<T> {
        first: T,
        second: T,
    }
    
    // Implementation for all types
    impl<T> Pair<T> {
        fn new(first: T, second: T) -> Self {
            Pair { first, second }
        }
    }
    
    // Implementation only for types that implement PartialOrd
    impl<T: PartialOrd> Pair<T> {
        fn larger(&self) -> &T {
            if self.first > self.second {
                &self.first
            } else {
                &self.second
            }
        }
    }
    
    // Implementation only for specific type
    impl Pair<i32> {
        fn sum(&self) -> i32 {
            self.first + self.second
        }
    }
    
    let int_pair = Pair::new(10, 20);
    let string_pair = Pair::new("hello".to_string(), "world".to_string());
    
    println!("Int pair: {:?}", int_pair);
    println!("String pair: {:?}", string_pair);
    
    println!("Larger int: {}", int_pair.larger());
    println!("Larger string: {}", string_pair.larger());
    
    println!("Sum of int pair: {}", int_pair.sum());
    // string_pair.sum(); // This won't compile - sum() only for i32
    println!();
}

/// Demo 4e: Associated Types and Advanced Traits
pub fn demo_associated_types() {
    println!("=== Demo 4e: Associated Types ===");
    
    trait Iterator {
        type Item;  // Associated type
        
        fn next(&mut self) -> Option<Self::Item>;
    }
    
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
    
    let mut counter = Counter::new(3);
    
    println!("Counter iteration:");
    while let Some(value) = counter.next() {
        println!("  {}", value);
    }
    println!();
}

/// Demo 4f: Operator Overloading with Traits
pub fn demo_operator_overloading() {
    println!("=== Demo 4f: Operator Overloading ===");
    
    use std::ops::Add;
    
    #[derive(Debug, Clone, Copy)]
    struct Point {
        x: i32,
        y: i32,
    }
    
    impl Add for Point {
        type Output = Point;
        
        fn add(self, other: Point) -> Point {
            Point {
                x: self.x + other.x,
                y: self.y + other.y,
            }
        }
    }
    
    let p1 = Point { x: 1, y: 2 };
    let p2 = Point { x: 3, y: 4 };
    let p3 = p1 + p2;  // Uses our Add implementation
    
    println!("p1: {:?}", p1);
    println!("p2: {:?}", p2);
    println!("p1 + p2 = {:?}", p3);
    println!();
}

/// Demo 4g: Common Standard Library Traits
pub fn demo_standard_traits() {
    println!("=== Demo 4g: Standard Library Traits ===");
    
    #[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
    struct Person {
        name: String,
        age: u32,
    }
    
    let person1 = Person {
        name: "Alice".to_string(),
        age: 30,
    };
    
    let person2 = person1.clone();  // Clone trait
    
    println!("person1: {:?}", person1);  // Debug trait
    println!("person2: {:?}", person2);
    
    println!("Are they equal? {}", person1 == person2);  // PartialEq trait
    
    let person3 = Person {
        name: "Bob".to_string(),
        age: 25,
    };
    
    println!("person1 > person3? {}", person1 > person3);  // PartialOrd trait
    
    // Into/From traits
    let s: String = "hello".into();  // &str into String
    println!("Converted string: {}", s);
    
    // Display trait implementation
    use std::fmt;
    
    impl fmt::Display for Person {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "{} (age {})", self.name, self.age)
        }
    }
    
    println!("Display: {}", person1);  // Display trait
    println!();
}

/// Run all demos in sequence
pub fn run_all_demos() {
    println!("🦀 RUST LECTURE - SECTION 4: TRAIT SYSTEM AND GENERICS 🦀");
    println!("===========================================================");
    println!();
    
    demo_basic_traits();
    demo_generic_functions();
    demo_trait_objects();
    demo_generic_structs();
    demo_associated_types();
    demo_operator_overloading();
    demo_standard_traits();
    
    println!("✅ Section 4 complete!");
    println!("💡 Key takeaway: Traits enable zero-cost abstractions and code reuse!");
}

/// Get list of available demos for enhanced navigation
pub fn get_demo_list() -> Vec<&'static str> {
    vec![
        "basic",
        "generics",
        "objects",
        "structs",
        "associated",
        "operators",
        "standard",
    ]
}
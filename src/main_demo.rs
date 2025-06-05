//! Interactive Rust Lecture Demo Runner
//! ====================================
//! 
//! Run this file to start the interactive lecture demonstration system.
//! You can run individual sections or demos during your live lecture.

use lecture::run_interactive_demo;

fn main() {
    // Start the interactive demo system
    run_interactive_demo();
}

// Alternative main functions for different use cases:

/// Run all sections sequentially (for a complete overview)
#[allow(dead_code)]
fn main_all_sections() {
    lecture::run_all_sections();
}

/// Example of running individual demos programmatically
#[allow(dead_code)]
fn main_individual_demos() {
    use lecture::individual_demos::*;
    use lecture::lecture_utils::*;
    
    print_section_separator("Custom Demo Sequence");
    
    // Run specific demos in a custom order
    println!("🎯 Running a custom sequence of demos...\n");
    
    lecture_pause("About to demonstrate basic Rust syntax");
    run_section1_demo("hello");
    run_section1_demo("variables");
    
    lecture_pause("Moving to ownership concepts");
    run_section2_demo("move");
    run_section2_demo("copy");
    
    lecture_pause("Exploring borrowing and references");
    run_section3_demo("immutable");
    run_section3_demo("mutable");
    
    highlight_concept(
        "Ownership System", 
        "Rust's ownership system prevents memory leaks and data races at compile time!"
    );
    
    println!("\n🎉 Custom demo sequence completed!");
}

/// Example of using the lecture utilities
#[allow(dead_code)]
fn main_with_utilities() {
    use lecture::lecture_utils::*;
    use lecture::section1_basics;
    
    print_section_separator("Rust Basics with Utilities");
    
    print_demo_separator("Hello World Demo");
    section1_basics::demo_hello_world();
    
    lecture_pause("Ready to see variables and mutability?");
    
    print_demo_separator("Variables and Mutability");
    section1_basics::demo_variables_mutability();
    
    highlight_concept(
        "Immutability by Default",
        "Rust variables are immutable by default, promoting safer code!"
    );
}
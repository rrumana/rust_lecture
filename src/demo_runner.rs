//! Demo Runner - Interactive Lecture Demonstration System
//! =====================================================
//! 
//! This module provides an interactive system for running individual
//! sections or demos during a live Rust lecture. Each section can be
//! demonstrated independently, making it perfect for live coding sessions.

#![allow(unused)]

// Import all section modules
use crate::section1_basics;
use crate::section2_ownership;
use crate::section3_borrowing;
use crate::section4_traits;
use crate::section5_enums;
use crate::section6_idioms;
use crate::section7_concurrency;
use crate::section8_crates;

use std::io::{self, Write};

/// Interactive menu system for running lecture demos
pub fn run_interactive_demo() {
    println!("🦀 RUST LECTURE DEMONSTRATION SYSTEM 🦀");
    println!("======================================");
    println!();
    
    loop {
        print_menu();
        
        let choice = get_user_input("Enter your choice (1-8, 'e' for enhanced, or 'q' to quit): ");
        
        match choice.trim() {
            "1" => {
                clear_screen();
                section1_basics::run_all_demos();
                wait_for_enter();
            }
            "2" => {
                clear_screen();
                section2_ownership::run_all_demos();
                wait_for_enter();
            }
            "3" => {
                clear_screen();
                section3_borrowing::run_all_demos();
                wait_for_enter();
            }
            "4" => {
                clear_screen();
                section4_traits::run_all_demos();
                wait_for_enter();
            }
            "5" => {
                clear_screen();
                section5_enums::run_all_demos();
                wait_for_enter();
            }
            "6" => {
                clear_screen();
                section6_idioms::run_all_demos();
                wait_for_enter();
            }
            "7" => {
                clear_screen();
                section7_concurrency::run_all_demos();
                wait_for_enter();
            }
            "8" => {
                clear_screen();
                section8_crates::run_all_demos();
                wait_for_enter();
            }
            "all" | "ALL" => {
                clear_screen();
                run_all_sections();
                wait_for_enter();
            }
            "e" | "E" | "enhanced" => {
                clear_screen();
                run_enhanced_navigation_mode();
            }
            "q" | "Q" | "quit" | "exit" => {
                println!("Thanks for using the Rust lecture demo system! 🦀");
                break;
            }
            _ => {
                println!("Invalid choice. Please try again.");
                wait_for_enter();
            }
        }
        
        clear_screen();
    }
}

/// Print the main menu
fn print_menu() {
    println!("📚 LECTURE SECTIONS:");
    println!("  1. Basic Syntax and Constructs");
    println!("  2. Ownership and Move Semantics");
    println!("  3. Borrowing, References, and Lifetimes");
    println!("  4. Trait System and Generics");
    println!("  5. Enums, Pattern Matching, Option & Result");
    println!("  6. Idiomatic Patterns & Utilities");
    println!("  7. Fearless Concurrency");
    println!("  8. Popular Crate Examples (20 crates)");
    println!();
    println!("🚀 SPECIAL OPTIONS:");
    println!("  all - Run all sections sequentially");
    println!("  e   - Enhanced navigation mode (individual demos)");
    println!("  q   - Quit");
    println!();
    println!("💡 Enhanced mode allows you to navigate individual demos with:");
    println!("   Enter = Next demo, Backspace = Previous demo");
    println!();
}

/// Get user input with a prompt
fn get_user_input(prompt: &str) -> String {
    print!("{}", prompt);
    io::stdout().flush().unwrap();
    
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input
}

/// Wait for user to press Enter
fn wait_for_enter() {
    println!();
    println!("Press Enter to continue...");
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
}

/// Clear the screen (works on most terminals)
fn clear_screen() {
    print!("\x1B[2J\x1B[1;1H");
    io::stdout().flush().unwrap();
}

/// Run all sections sequentially
pub fn run_all_sections() {
    println!("🦀 RUNNING ALL LECTURE SECTIONS 🦀");
    println!("==================================");
    println!();
    
    section1_basics::run_all_demos();
    println!("\n{}\n", "=".repeat(60));
    
    section2_ownership::run_all_demos();
    println!("\n{}\n", "=".repeat(60));
    
    section3_borrowing::run_all_demos();
    println!("\n{}\n", "=".repeat(60));
    
    section4_traits::run_all_demos();
    println!("\n{}\n", "=".repeat(60));
    
    section5_enums::run_all_demos();
    println!("\n{}\n", "=".repeat(60));
    
    section6_idioms::run_all_demos();
    println!("\n{}\n", "=".repeat(60));
    
    section7_concurrency::run_all_demos();
    println!("\n{}\n", "=".repeat(60));
    
    section8_crates::run_all_demos();
    
    println!("\n🎉 ALL SECTIONS COMPLETED! 🎉");
    println!("You've seen a comprehensive overview of Rust!");
}


/// Individual demo runners for fine-grained control during lectures
pub mod individual_demos {
    use super::*;
    
    /// Run a specific demo from section 1
    pub fn run_section1_demo(demo_name: &str) {
        match demo_name {
            "hello" => section1_basics::demo_hello_world(),
            "variables" => section1_basics::demo_variables_mutability(),
            "functions" => section1_basics::demo_functions(),
            "if" => section1_basics::demo_if_expressions(),
            "match" => section1_basics::demo_match_expressions(),
            "for" => section1_basics::demo_for_loops(),
            "while" => section1_basics::demo_while_loops(),
            "blocks" => section1_basics::demo_block_expressions(),
            _ => println!("Unknown demo: {}", demo_name),
        }
    }
    
    /// Run a specific demo from section 2
    pub fn run_section2_demo(demo_name: &str) {
        match demo_name {
            "scope" => section2_ownership::demo_ownership_scope(),
            "move" => section2_ownership::demo_move_semantics(),
            "copy" => section2_ownership::demo_copy_types(),
            "functions" => section2_ownership::demo_function_ownership(),
            "collections" => section2_ownership::demo_collection_ownership(),
            "patterns" => section2_ownership::demo_ownership_patterns(),
            _ => println!("Unknown demo: {}", demo_name),
        }
    }
    
    /// Run a specific demo from section 3
    pub fn run_section3_demo(demo_name: &str) {
        match demo_name {
            "immutable" => section3_borrowing::demo_immutable_borrowing(),
            "mutable" => section3_borrowing::demo_mutable_borrowing(),
            "rules" => section3_borrowing::demo_borrowing_rules(),
            "lifetimes" => section3_borrowing::demo_lifetimes(),
            "elision" => section3_borrowing::demo_lifetime_elision(),
            "patterns" => section3_borrowing::demo_reference_patterns(),
            "dangling" => section3_borrowing::demo_dangling_prevention(),
            _ => println!("Unknown demo: {}", demo_name),
        }
    }
    
    /// Run a specific demo from section 4
    pub fn run_section4_demo(demo_name: &str) {
        match demo_name {
            "basic" => section4_traits::demo_basic_traits(),
            "generics" => section4_traits::demo_generic_functions(),
            "objects" => section4_traits::demo_trait_objects(),
            "structs" => section4_traits::demo_generic_structs(),
            "associated" => section4_traits::demo_associated_types(),
            "operators" => section4_traits::demo_operator_overloading(),
            "standard" => section4_traits::demo_standard_traits(),
            _ => println!("Unknown demo: {}", demo_name),
        }
    }
    
    /// Run a specific demo from section 5
    pub fn run_section5_demo(demo_name: &str) {
        match demo_name {
            "basic" => section5_enums::demo_basic_enums(),
            "data" => section5_enums::demo_enums_with_data(),
            "option" => section5_enums::demo_option_type(),
            "result" => section5_enums::demo_result_type(),
            "patterns" => section5_enums::demo_advanced_patterns(),
            "recursive" => section5_enums::demo_recursive_enums(),
            "propagation" => section5_enums::demo_error_propagation(),
            _ => println!("Unknown demo: {}", demo_name),
        }
    }
    
    /// Run a specific demo from section 6
    pub fn run_section6_demo(demo_name: &str) {
        match demo_name {
            "iterators" => section6_idioms::demo_iterator_patterns(),
            "advanced_iterators" => section6_idioms::demo_advanced_iterators(),
            "errors" => section6_idioms::demo_error_handling_patterns(),
            "shadowing" => section6_idioms::demo_shadowing_patterns(),
            "memory" => section6_idioms::demo_memory_patterns(),
            "utilities" => section6_idioms::demo_utility_patterns(),
            _ => println!("Unknown demo: {}", demo_name),
        }
    }
    
    /// Run a specific demo from section 7
    pub fn run_section7_demo(demo_name: &str) {
        match demo_name {
            "threading" => section7_concurrency::demo_basic_threading(),
            "channels" => section7_concurrency::demo_message_passing(),
            "shared" => section7_concurrency::demo_shared_state(),
            "advanced" => section7_concurrency::demo_advanced_concurrency(),
            "async" => section7_concurrency::demo_async_basics(),
            "safety" => section7_concurrency::demo_thread_safety(),
            _ => println!("Unknown demo: {}", demo_name),
        }
    }
    
    /// Run a specific demo from section 8
    pub fn run_section8_demo(demo_name: &str) {
        match demo_name {
            "1" | "serde" => section8_crates::demo_1_serde_json(),
            "2" | "rand" => section8_crates::demo_2_rand(),
            "3" | "clap" => section8_crates::demo_3_clap(),
            "4" | "tokio" => section8_crates::demo_4_tokio(),
            "5" | "reqwest" => section8_crates::demo_5_reqwest(),
            "6" | "regex" => section8_crates::demo_6_regex(),
            "7" | "chrono" => section8_crates::demo_7_chrono(),
            "8" | "anyhow" => section8_crates::demo_8_anyhow(),
            "9" | "thiserror" => section8_crates::demo_9_thiserror(),
            "10" | "crossbeam" => section8_crates::demo_10_crossbeam(),
            "11" | "rayon" => section8_crates::demo_11_rayon(),
            "12" | "tracing" => section8_crates::demo_12_tracing(),
            "13" | "log" => section8_crates::demo_13_log(),
            "14" | "itertools" => section8_crates::demo_14_itertools(),
            "15" | "once_cell" => section8_crates::demo_15_once_cell(),
            "16" | "uuid" => section8_crates::demo_16_uuid(),
            "17" | "tempfile" => section8_crates::demo_17_tempfile(),
            "18" | "bitflags" => section8_crates::demo_18_bitflags(),
            "19" | "parking_lot" => section8_crates::demo_19_parking_lot(),
            "20" | "collections" => section8_crates::demo_20_advanced_collections(),
            _ => println!("Unknown demo: {}", demo_name),
        }
    }
    
    /// Print available demos for a section
    pub fn print_section_demos(section: u8) {
        match section {
            1 => {
                println!("Available Section 1 demos:");
                println!("  hello, variables, functions, if, match, for, while, blocks");
            }
            2 => {
                println!("Available Section 2 demos:");
                println!("  scope, move, copy, functions, collections, patterns");
            }
            3 => {
                println!("Available Section 3 demos:");
                println!("  immutable, mutable, rules, lifetimes, elision, patterns, dangling");
            }
            4 => {
                println!("Available Section 4 demos:");
                println!("  basic, generics, objects, structs, associated, operators, standard");
            }
            5 => {
                println!("Available Section 5 demos:");
                println!("  basic, data, option, result, patterns, recursive, propagation");
            }
            6 => {
                println!("Available Section 6 demos:");
                println!("  iterators, advanced_iterators, errors, shadowing, memory, utilities");
            }
            7 => {
                println!("Available Section 7 demos:");
                println!("  threading, channels, shared, advanced, async, safety");
            }
            8 => {
                println!("Available Section 8 demos:");
                println!("  1/serde - JSON serialization with serde");
                println!("  2/rand - Random number generation");
                println!("  3/clap - Command-line argument parsing");
                println!("  4/tokio - Async runtime and tasks");
                println!("  5/reqwest - HTTP client requests");
                println!("  6/regex - Regular expression matching");
                println!("  7/chrono - Date and time handling");
                println!("  8/anyhow - Flexible error handling");
                println!("  9/thiserror - Custom error types");
                println!("  10/crossbeam - Lock-free data structures");
                println!("  11/rayon - Data parallelism");
                println!("  12/tracing - Structured logging");
                println!("  13/log - Simple logging");
                println!("  14/itertools - Extended iterator methods");
                println!("  15/once_cell - Lazy static initialization");
                println!("  16/uuid - UUID generation");
                println!("  17/tempfile - Temporary file management");
                println!("  18/bitflags - Bit flag operations");
                println!("  19/parking_lot - High-performance synchronization");
                println!("  20/collections - Advanced collection types");
            }
            _ => println!("Invalid section number. Use 1-8."),
        }
    }
}

/// Utility functions for lecture management
pub mod lecture_utils {
    /// Print a section separator
    pub fn print_section_separator(section_name: &str) {
        let separator = "=".repeat(60);
        println!("\n{}", separator);
        println!("🦀 {}", section_name.to_uppercase());
        println!("{}\n", separator);
    }
    
    /// Print a demo separator
    pub fn print_demo_separator(demo_name: &str) {
        println!("\n{}", "-".repeat(40));
        println!("📍 {}", demo_name);
        println!("{}", "-".repeat(40));
    }
    
    /// Create a pause for live demonstrations
    pub fn lecture_pause(message: &str) {
        println!("\n⏸️  LECTURE PAUSE: {}", message);
        println!("   Press Enter when ready to continue...");
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
    }
    
    /// Highlight important concepts
    pub fn highlight_concept(concept: &str, explanation: &str) {
        println!("\n💡 KEY CONCEPT: {}", concept);
        println!("   {}", explanation);
        println!();
    }
}

/// Enhanced navigation mode for individual demo control
pub fn run_enhanced_navigation_mode() {
    println!("🦀 ENHANCED NAVIGATION MODE 🦀");
    println!("==============================");
    println!();
    println!("Choose a section to navigate through individual demos:");
    println!("  1. Basic Syntax and Constructs (8 demos)");
    println!("  2. Ownership and Move Semantics (6 demos)");
    println!("  3. Borrowing, References, and Lifetimes (7 demos)");
    println!("  4. Trait System and Generics (7 demos)");
    println!("  5. Enums, Pattern Matching, Option & Result (7 demos)");
    println!("  6. Idiomatic Patterns & Utilities (6 demos)");
    println!("  7. Fearless Concurrency (6 demos)");
    println!("  8. Popular Crate Examples (20 demos)");
    println!("  all - Navigate through all demos sequentially");
    println!("  q   - Return to main menu");
    println!();
    
    let choice = get_user_input("Enter your choice: ");
    
    match choice.trim() {
        "1" => run_section_enhanced_navigation(1),
        "2" => run_section_enhanced_navigation(2),
        "3" => run_section_enhanced_navigation(3),
        "4" => run_section_enhanced_navigation(4),
        "5" => run_section_enhanced_navigation(5),
        "6" => run_section_enhanced_navigation(6),
        "7" => run_section_enhanced_navigation(7),
        "8" => run_section_enhanced_navigation(8),
        "all" | "ALL" => run_all_demos_enhanced_navigation(),
        "q" | "Q" => return,
        _ => {
            println!("Invalid choice. Returning to main menu.");
            wait_for_enter();
        }
    }
}

/// Run enhanced navigation for a specific section
fn run_section_enhanced_navigation(section: u8) {
    let demos = get_section_demo_list(section);
    if demos.is_empty() {
        println!("No demos found for section {}", section);
        wait_for_enter();
        return;
    }
    
    clear_screen();
    println!("🦀 SECTION {} - ENHANCED NAVIGATION 🦀", section);
    println!("=====================================");
    println!();
    println!("Controls:");
    println!("  Enter = Next demo");
    println!("  'p'   = Previous demo");
    println!("  'q'   = Quit to main menu");
    println!();
    
    let mut current_index = 0;
    
    loop {
        // Run the current demo
        run_individual_demo(section, demos[current_index]);
        
        // Get navigation input
        println!("\n⌨️  Navigation: [Enter]=Next ['p']=Previous ['q']=Quit");
        match get_enhanced_navigation_input() {
            NavigationAction::Next => {
                if current_index < demos.len() - 1 {
                    current_index += 1;
                    clear_screen();
                } else {
                    println!("\n🎉 You've reached the end of Section {}!", section);
                    println!("Press Enter to return to menu, or 'p' to go back.");
                    match get_enhanced_navigation_input() {
                        NavigationAction::Previous => {
                            current_index = demos.len() - 1;
                            clear_screen();
                        }
                        _ => break,
                    }
                }
            }
            NavigationAction::Previous => {
                if current_index > 0 {
                    current_index -= 1;
                    clear_screen();
                } else {
                    println!("\n📍 You're at the beginning of Section {}!", section);
                    println!("Press Enter to continue, or 'q' to quit.");
                    match get_enhanced_navigation_input() {
                        NavigationAction::Quit => break,
                        _ => clear_screen(),
                    }
                }
            }
            NavigationAction::Quit => break,
        }
    }
}

/// Run all demos with enhanced navigation
fn run_all_demos_enhanced_navigation() {
    let mut all_demos = Vec::new();
    
    // Collect all demos from all sections
    for section in 1..=8 {
        let section_demos = get_section_demo_list(section);
        for demo_name in section_demos {
            all_demos.push((section, demo_name));
        }
    }
    
    if all_demos.is_empty() {
        println!("No demos found!");
        wait_for_enter();
        return;
    }
    
    clear_screen();
    println!("🦀 ALL SECTIONS - ENHANCED NAVIGATION 🦀");
    println!("========================================");
    println!();
    println!("Controls:");
    println!("  Enter = Next demo");
    println!("  'p'   = Previous demo");
    println!("  'q'   = Quit to main menu");
    println!();
    
    let mut current_index = 0;
    
    loop {
        // Run the current demo
        let (section, demo_name) = &all_demos[current_index];
        run_individual_demo(*section, demo_name);
        
        // Get navigation input
        println!("\n⌨️  Navigation: [Enter]=Next ['p']=Previous ['q']=Quit");
        match get_enhanced_navigation_input() {
            NavigationAction::Next => {
                if current_index < all_demos.len() - 1 {
                    current_index += 1;
                    clear_screen();
                } else {
                    println!("\n🎉 You've completed all demos!");
                    println!("Press Enter to return to menu, or 'p' to go back.");
                    match get_enhanced_navigation_input() {
                        NavigationAction::Previous => {
                            current_index = all_demos.len() - 1;
                            clear_screen();
                        }
                        _ => break,
                    }
                }
            }
            NavigationAction::Previous => {
                if current_index > 0 {
                    current_index -= 1;
                    clear_screen();
                } else {
                    println!("\n📍 You're at the beginning!");
                    println!("Press Enter to continue, or 'q' to quit.");
                    match get_enhanced_navigation_input() {
                        NavigationAction::Quit => break,
                        _ => clear_screen(),
                    }
                }
            }
            NavigationAction::Quit => break,
        }
    }
}

/// Navigation actions for enhanced mode
#[derive(Debug)]
enum NavigationAction {
    Next,
    Previous,
    Quit,
}

/// Get enhanced navigation input from user
fn get_enhanced_navigation_input() -> NavigationAction {
    print!("Your choice: ");
    io::stdout().flush().unwrap();
    
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    
    match input.trim() {
        "" => NavigationAction::Next,  // Enter key
        "q" | "Q" | "quit" => NavigationAction::Quit,
        "p" | "P" | "prev" | "previous" => NavigationAction::Previous,
        _ => NavigationAction::Next,  // Default to next for any other input
    }
}

/// Get demo list for a section
fn get_section_demo_list(section: u8) -> Vec<&'static str> {
    match section {
        1 => section1_basics::get_demo_list(),
        2 => section2_ownership::get_demo_list(),
        3 => section3_borrowing::get_demo_list(),
        4 => section4_traits::get_demo_list(),
        5 => section5_enums::get_demo_list(),
        6 => section6_idioms::get_demo_list(),
        7 => section7_concurrency::get_demo_list(),
        8 => section8_crates::get_demo_list(),
        _ => vec![],
    }
}

/// Run an individual demo
fn run_individual_demo(section: u8, demo_name: &str) {
    match section {
        1 => individual_demos::run_section1_demo(demo_name),
        2 => individual_demos::run_section2_demo(demo_name),
        3 => individual_demos::run_section3_demo(demo_name),
        4 => individual_demos::run_section4_demo(demo_name),
        5 => individual_demos::run_section5_demo(demo_name),
        6 => individual_demos::run_section6_demo(demo_name),
        7 => individual_demos::run_section7_demo(demo_name),
        8 => individual_demos::run_section8_demo(demo_name),
        _ => println!("Unknown section: {}", section),
    }
}
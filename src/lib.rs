//! Rust Lecture Demonstration Library
//! ==================================
//!
//! This library provides a comprehensive set of Rust examples organized
//! into sections that can be demonstrated during live lectures.

pub mod demo_runner;
pub mod section1_basics;
pub mod section2_ownership;
pub mod section3_borrowing;
pub mod section4_traits;
pub mod section5_enums;
pub mod section6_idioms;
pub mod section7_concurrency;
pub mod section8_crates;

// Re-export the main demo runner for easy access
pub use demo_runner::run_interactive_demo;
pub use demo_runner::run_all_sections;
pub use demo_runner::individual_demos;
pub use demo_runner::lecture_utils;
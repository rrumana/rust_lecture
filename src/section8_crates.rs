//! Section 8: Popular Rust Crates - Real Working Examples
//! ======================================================
//! 
//! This section demonstrates 20 essential Rust crates with actual working code.
//! Each example shows practical usage patterns that you would use in real applications.

#![allow(unused)]

use std::collections::HashMap;
use std::time::Duration;
use std::error::Error;

/// Demo 1: Serde + serde_json - Serialization and Deserialization
pub fn demo_1_serde_json() {
    println!("=== Demo 1: Serde + serde_json ===");
    
    use serde::{Serialize, Deserialize};
    
    #[derive(Serialize, Deserialize, Debug, Clone)]
    struct User {
        name: String,
        age: u8,
        email: String,
        active: bool,
    }
    
    #[derive(Serialize, Deserialize, Debug)]
    struct ApiResponse {
        users: Vec<User>,
        total: usize,
        page: u32,
    }
    
    // Create some sample data
    let users = vec![
        User {
            name: "Alice Johnson".to_string(),
            age: 30,
            email: "alice@example.com".to_string(),
            active: true,
        },
        User {
            name: "Bob Smith".to_string(),
            age: 25,
            email: "bob@example.com".to_string(),
            active: false,
        },
    ];
    
    let response = ApiResponse {
        users: users.clone(),
        total: users.len(),
        page: 1,
    };
    
    // Serialize to JSON
    let json = serde_json::to_string_pretty(&response).unwrap();
    println!("Serialized JSON:\n{}", json);
    
    // Deserialize back from JSON
    let parsed: ApiResponse = serde_json::from_str(&json).unwrap();
    println!("Deserialized: {} users on page {}", parsed.total, parsed.page);
    
    // Working with JSON values directly
    let json_value: serde_json::Value = serde_json::from_str(&json).unwrap();
    if let Some(first_user) = json_value["users"][0]["name"].as_str() {
        println!("First user name: {}", first_user);
    }
    
    println!();
}

/// Demo 2: Rand - Random Number Generation
pub fn demo_2_rand() {
    println!("=== Demo 2: Rand ===");
    
    use rand::prelude::*;
    use rand::distributions::{Alphanumeric, Uniform};
    
    let mut rng = thread_rng();
    
    // Basic random numbers
    println!("Random u32: {}", rng.r#gen::<u32>());
    println!("Random f64 [0,1): {:.4}", rng.r#gen::<f64>());
    println!("Random bool: {}", rng.r#gen::<bool>());
    
    // Random in range
    println!("Random 1-100: {}", rng.gen_range(1..=100));
    println!("Random dice roll: {}", rng.gen_range(1..=6));
    
    // Random choice from slice
    let colors = ["red", "green", "blue", "yellow", "purple"];
    println!("Random color: {}", colors.choose(&mut rng).unwrap());
    
    // Shuffle a vector
    let mut numbers: Vec<i32> = (1..=10).collect();
    numbers.shuffle(&mut rng);
    println!("Shuffled numbers: {:?}", numbers);
    
    // Generate random string
    let random_string: String = (&mut rng)
        .sample_iter(&Alphanumeric)
        .take(10)
        .map(char::from)
        .collect();
    println!("Random string: {}", random_string);
    
    // Using distributions
    let uniform = Uniform::from(10..20);
    let samples: Vec<i32> = (0..5).map(|_| uniform.sample(&mut rng)).collect();
    println!("Uniform samples [10,20): {:?}", samples);
    
    println!();
}

/// Demo 3: Clap - Command Line Argument Parsing
pub fn demo_3_clap() {
    println!("=== Demo 3: Clap ===");
    
    use clap::{Arg, Command, ArgMatches};
    
    // Create a CLI application
    let app = Command::new("myapp")
        .version("1.0")
        .author("Your Name <you@example.com>")
        .about("Demonstrates clap argument parsing")
        .arg(
            Arg::new("input")
                .short('i')
                .long("input")
                .value_name("FILE")
                .help("Sets the input file to use")
                .required(false)
        )
        .arg(
            Arg::new("verbose")
                .short('v')
                .long("verbose")
                .help("Enable verbose output")
                .action(clap::ArgAction::SetTrue)
        )
        .arg(
            Arg::new("count")
                .short('c')
                .long("count")
                .value_name("NUMBER")
                .help("Number of iterations")
                .value_parser(clap::value_parser!(u32))
        )
        .subcommand(
            Command::new("process")
                .about("Process data")
                .arg(
                    Arg::new("format")
                        .long("format")
                        .value_name("FORMAT")
                        .help("Output format")
                        .value_parser(["json", "xml", "csv"])
                )
        );
    
    // Simulate different command line arguments
    let test_args = vec![
        vec!["myapp", "--verbose", "--count", "5", "--input", "data.txt"],
        vec!["myapp", "process", "--format", "json"],
        vec!["myapp", "--help"],
    ];
    
    for args in test_args {
        println!("Parsing: {}", args.join(" "));
        
        // Skip the help case as it would exit
        if args.contains(&"--help") {
            println!("  Would show help and exit");
            continue;
        }
        
        match app.clone().try_get_matches_from(args) {
            Ok(matches) => {
                if matches.get_flag("verbose") {
                    println!("  Verbose mode enabled");
                }
                
                if let Some(input) = matches.get_one::<String>("input") {
                    println!("  Input file: {}", input);
                }
                
                if let Some(count) = matches.get_one::<u32>("count") {
                    println!("  Count: {}", count);
                }
                
                if let Some(subcommand) = matches.subcommand() {
                    match subcommand {
                        ("process", sub_matches) => {
                            println!("  Processing data");
                            if let Some(format) = sub_matches.get_one::<String>("format") {
                                println!("  Output format: {}", format);
                            }
                        }
                        _ => {}
                    }
                }
            }
            Err(e) => {
                println!("  Error: {}", e);
            }
        }
        println!();
    }
}

/// Demo 4: Tokio - Asynchronous Runtime (simplified for demo)
pub fn demo_4_tokio() {
    println!("=== Demo 4: Tokio ===");
    
    use tokio::time::{sleep, timeout, Instant};
    use tokio::task;
    
    // Create a simple async runtime for demonstration
    let rt = tokio::runtime::Runtime::new().unwrap();
    
    rt.block_on(async {
        println!("Running async tasks...");
        
        // Basic async/await
        async fn fetch_data(id: u32) -> String {
            sleep(Duration::from_millis(100)).await;
            format!("Data for ID: {}", id)
        }
        
        let start = Instant::now();
        let data = fetch_data(42).await;
        println!("Sequential: {} (took {:?})", data, start.elapsed());
        
        // Concurrent execution
        let start = Instant::now();
        let tasks = (1..=3).map(|id| task::spawn(fetch_data(id)));
        
        let results: Vec<String> = futures::future::join_all(tasks)
            .await
            .into_iter()
            .map(|r| r.unwrap())
            .collect();
        
        println!("Concurrent results: {:?} (took {:?})", results, start.elapsed());
        
        // Timeout example
        async fn slow_operation() -> &'static str {
            sleep(Duration::from_millis(200)).await;
            "Completed"
        }
        
        match timeout(Duration::from_millis(100), slow_operation()).await {
            Ok(result) => println!("Operation completed: {}", result),
            Err(_) => println!("Operation timed out"),
        }
        
        // Task spawning
        let handle = task::spawn(async {
            for i in 1..=3 {
                println!("Background task iteration {}", i);
                sleep(Duration::from_millis(50)).await;
            }
            "Background task completed"
        });
        
        let result = handle.await.unwrap();
        println!("{}", result);
    });
    
    println!();
}

/// Demo 5: Reqwest - HTTP Client
pub fn demo_5_reqwest() {
    println!("=== Demo 5: Reqwest ===");
    
    // Using blocking client for simplicity in demo
    use reqwest::blocking::Client;
    use serde_json::Value;
    
    let client = Client::new();
    
    // Simple GET request
    println!("Making HTTP requests...");
    
    // GET request to a JSON API
    match client.get("https://httpbin.org/json").send() {
        Ok(response) => {
            println!("Status: {}", response.status());
            println!("Headers: {:#?}", response.headers().get("content-type"));
            
            if let Ok(json) = response.json::<Value>() {
                if let Some(slideshow) = json.get("slideshow") {
                    println!("Response data: {}", slideshow);
                }
            }
        }
        Err(e) => println!("Request failed: {}", e),
    }
    
    // POST request with JSON body
    let post_data = serde_json::json!({
        "name": "John Doe",
        "email": "john@example.com"
    });
    
    match client
        .post("https://httpbin.org/post")
        .json(&post_data)
        .send()
    {
        Ok(response) => {
            println!("POST Status: {}", response.status());
            if let Ok(json) = response.json::<Value>() {
                if let Some(data) = json.get("json") {
                    println!("Echoed data: {}", data);
                }
            }
        }
        Err(e) => println!("POST request failed: {}", e),
    }
    
    // Request with custom headers
    match client
        .get("https://httpbin.org/headers")
        .header("User-Agent", "Rust-Demo/1.0")
        .header("X-Custom-Header", "demo-value")
        .send()
    {
        Ok(response) => {
            if let Ok(json) = response.json::<Value>() {
                if let Some(headers) = json.get("headers") {
                    println!("Server saw headers: {}", headers);
                }
            }
        }
        Err(e) => println!("Headers request failed: {}", e),
    }
    
    println!();
}

/// Demo 6: Regex - Regular Expressions
pub fn demo_6_regex() {
    println!("=== Demo 6: Regex ===");
    
    use regex::Regex;
    
    // Email validation
    let email_regex = Regex::new(r"^[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}$").unwrap();
    
    let emails = [
        "valid@example.com",
        "also.valid+tag@domain.co.uk",
        "invalid.email",
        "missing@.com",
    ];
    
    println!("Email validation:");
    for email in emails {
        let is_valid = email_regex.is_match(email);
        println!("  {}: {}", email, if is_valid { "✓ Valid" } else { "✗ Invalid" });
    }
    
    // Extracting data with capture groups
    let log_regex = Regex::new(r"(\d{4}-\d{2}-\d{2}) (\d{2}:\d{2}:\d{2}) \[(\w+)\] (.+)").unwrap();
    
    let log_lines = [
        "2025-06-04 14:30:15 [INFO] User logged in",
        "2025-06-04 14:31:22 [ERROR] Database connection failed",
        "2025-06-04 14:32:01 [WARN] High memory usage detected",
    ];
    
    println!("\nLog parsing:");
    for line in log_lines {
        if let Some(caps) = log_regex.captures(line) {
            println!("  Date: {}, Time: {}, Level: {}, Message: {}", 
                    &caps[1], &caps[2], &caps[3], &caps[4]);
        }
    }
    
    // Named capture groups
    let phone_regex = Regex::new(r"(?P<area>\d{3})-(?P<exchange>\d{3})-(?P<number>\d{4})").unwrap();
    
    let text = "Call me at 555-123-4567 or 800-555-0199";
    println!("\nPhone number extraction:");
    for caps in phone_regex.captures_iter(text) {
        println!("  Found: ({}) {}-{}", &caps["area"], &caps["exchange"], &caps["number"]);
    }
    
    // Find and replace
    let text = "The quick brown fox jumps over the lazy dog";
    let animal_regex = Regex::new(r"\b(fox|dog)\b").unwrap();
    let replaced = animal_regex.replace_all(text, "animal");
    println!("\nFind and replace:");
    println!("  Original: {}", text);
    println!("  Replaced: {}", replaced);
    
    println!();
}

/// Demo 7: Chrono - Date and Time Handling
pub fn demo_7_chrono() {
    println!("=== Demo 7: Chrono ===");
    
    use chrono::{DateTime, Local, Utc, NaiveDate, Duration, TimeZone};
    
    // Current time
    let now_local: DateTime<Local> = Local::now();
    let now_utc: DateTime<Utc> = Utc::now();
    
    println!("Current time:");
    println!("  Local: {}", now_local.format("%Y-%m-%d %H:%M:%S %Z"));
    println!("  UTC:   {}", now_utc.format("%Y-%m-%d %H:%M:%S %Z"));
    
    // Parsing dates
    let date_str = "2025-06-04";
    let parsed_date = NaiveDate::parse_from_str(date_str, "%Y-%m-%d").unwrap();
    println!("\nParsed date: {} -> {}", date_str, parsed_date.format("%B %d, %Y"));
    
    // Date arithmetic
    let tomorrow = now_local + Duration::days(1);
    let last_week = now_local - Duration::weeks(1);
    let in_30_days = now_local + Duration::days(30);
    
    println!("\nDate arithmetic:");
    println!("  Tomorrow: {}", tomorrow.format("%Y-%m-%d"));
    println!("  Last week: {}", last_week.format("%Y-%m-%d"));
    println!("  In 30 days: {}", in_30_days.format("%Y-%m-%d"));
    
    // Duration calculations
    let birthday = Local.with_ymd_and_hms(1990, 6, 15, 0, 0, 0).unwrap();
    let age = now_local.signed_duration_since(birthday);
    println!("\nAge calculation:");
    println!("  Days since birthday: {}", age.num_days());
    println!("  Years (approx): {:.1}", age.num_days() as f64 / 365.25);
    
    // Working with different time zones
    let utc_time = Utc::now();
    let tokyo_offset = chrono::FixedOffset::east_opt(9 * 3600).unwrap(); // UTC+9
    let tokyo_time = utc_time.with_timezone(&tokyo_offset);
    
    println!("\nTime zones:");
    println!("  UTC: {}", utc_time.format("%H:%M:%S"));
    println!("  Tokyo: {}", tokyo_time.format("%H:%M:%S"));
    
    // Formatting examples
    println!("\nFormatting examples:");
    println!("  ISO 8601: {}", now_local.to_rfc3339());
    println!("  RFC 2822: {}", now_local.to_rfc2822());
    println!("  Custom: {}", now_local.format("%A, %B %d, %Y at %I:%M %p"));
    
    println!();
}

/// Demo 8: Anyhow - Error Handling with Context
pub fn demo_8_anyhow() {
    println!("=== Demo 8: Anyhow ===");
    
    use anyhow::{Result, Context, bail};
    use std::fs;
    
    fn read_config_file(path: &str) -> Result<String> {
        fs::read_to_string(path)
            .with_context(|| format!("Failed to read config file: {}", path))
    }
    
    fn parse_config(content: &str) -> Result<HashMap<String, String>> {
        let mut config = HashMap::new();
        
        for (line_num, line) in content.lines().enumerate() {
            let line = line.trim();
            if line.is_empty() || line.starts_with('#') {
                continue;
            }
            
            if let Some((key, value)) = line.split_once('=') {
                config.insert(key.trim().to_string(), value.trim().to_string());
            } else {
                bail!("Invalid config format at line {}: {}", line_num + 1, line);
            }
        }
        
        Ok(config)
    }
    
    fn load_and_parse_config(path: &str) -> Result<HashMap<String, String>> {
        let content = read_config_file(path)
            .context("Configuration loading failed")?;
        
        let config = parse_config(&content)
            .context("Configuration parsing failed")?;
        
        if config.is_empty() {
            bail!("Configuration file is empty or contains no valid entries");
        }
        
        Ok(config)
    }
    
    // Test with different scenarios
    println!("Testing error handling scenarios:");
    
    // Scenario 1: File doesn't exist
    match load_and_parse_config("nonexistent.conf") {
        Ok(config) => println!("  Unexpected success: {:?}", config),
        Err(e) => {
            println!("  Error chain for missing file:");
            for (i, cause) in e.chain().enumerate() {
                println!("    {}: {}", i, cause);
            }
        }
    }
    
    // Scenario 2: Create a temporary config file
    use std::io::Write;
    let temp_config = "
# Sample configuration
database_url=postgresql://localhost/myapp
api_key=secret123
debug=true
invalid_line_without_equals
port=8080
";
    
    let temp_path = "/tmp/demo_config.conf";
    if let Ok(mut file) = std::fs::File::create(temp_path) {
        let _ = file.write_all(temp_config.as_bytes());
        
        match load_and_parse_config(temp_path) {
            Ok(config) => println!("  Successfully loaded config: {:?}", config),
            Err(e) => {
                println!("  Error with invalid config:");
                for (i, cause) in e.chain().enumerate() {
                    println!("    {}: {}", i, cause);
                }
            }
        }
        
        // Clean up
        let _ = std::fs::remove_file(temp_path);
    }
    
    // Demonstrate error conversion
    fn network_operation() -> Result<String> {
        // Simulate a network error
        std::fs::read_to_string("nonexistent_network_resource")
            .context("Network operation failed")
            .context("Service unavailable")
    }
    
    match network_operation() {
        Ok(data) => println!("  Network data: {}", data),
        Err(e) => {
            println!("  Network error chain:");
            for (i, cause) in e.chain().enumerate() {
                println!("    {}: {}", i, cause);
            }
        }
    }
    
    println!();
}

/// Demo 9: Thiserror - Custom Error Types
pub fn demo_9_thiserror() {
    println!("=== Demo 9: Thiserror ===");
    
    use thiserror::Error;
    
    #[derive(Error, Debug)]
    enum DataProcessingError {
        #[error("Invalid input format: {message}")]
        InvalidFormat { message: String },
        
        #[error("Network timeout after {timeout_ms}ms")]
        NetworkTimeout { timeout_ms: u64 },
        
        #[error("Database error: {0}")]
        Database(String),
        
        #[error("IO error")]
        Io(#[from] std::io::Error),
        
        #[error("JSON parsing error")]
        Json(#[from] serde_json::Error),
        
        #[error("Validation failed")]
        Validation {
            #[source]
            source: ValidationError,
            field: String,
        },
    }
    
    #[derive(Error, Debug)]
    enum ValidationError {
        #[error("Value too small: {value} < {min}")]
        TooSmall { value: i32, min: i32 },
        
        #[error("Value too large: {value} > {max}")]
        TooLarge { value: i32, max: i32 },
        
        #[error("Invalid email format")]
        InvalidEmail,
    }
    
    fn validate_age(age: i32) -> Result<(), ValidationError> {
        if age < 0 {
            Err(ValidationError::TooSmall { value: age, min: 0 })
        } else if age > 150 {
            Err(ValidationError::TooLarge { value: age, max: 150 })
        } else {
            Ok(())
        }
    }
    
    fn validate_email(email: &str) -> Result<(), ValidationError> {
        if email.contains('@') && email.contains('.') {
            Ok(())
        } else {
            Err(ValidationError::InvalidEmail)
        }
    }
    
    fn process_user_data(name: &str, age: i32, email: &str) -> Result<String, DataProcessingError> {
        if name.is_empty() {
            return Err(DataProcessingError::InvalidFormat {
                message: "Name cannot be empty".to_string(),
            });
        }
        
        validate_age(age).map_err(|e| DataProcessingError::Validation {
            source: e,
            field: "age".to_string(),
        })?;
        
        validate_email(email).map_err(|e| DataProcessingError::Validation {
            source: e,
            field: "email".to_string(),
        })?;
        
        // Simulate network timeout
        if name == "timeout" {
            return Err(DataProcessingError::NetworkTimeout { timeout_ms: 5000 });
        }
        
        // Simulate database error
        if name == "dberror" {
            return Err(DataProcessingError::Database("Connection refused".to_string()));
        }
        
        Ok(format!("Processed user: {} (age: {}, email: {})", name, age, email))
    }
    
    // Test different error scenarios
    let test_cases = [
        ("Alice", 30, "alice@example.com"),
        ("", 25, "test@example.com"),
        ("Bob", -5, "bob@example.com"),
        ("Charlie", 200, "charlie@example.com"),
        ("Dave", 35, "invalid-email"),
        ("timeout", 30, "timeout@example.com"),
        ("dberror", 25, "db@example.com"),
    ];
    
    println!("Testing custom error types:");
    for (name, age, email) in test_cases {
        match process_user_data(name, age, email) {
            Ok(result) => println!("  ✓ {}", result),
            Err(e) => {
                println!("  ✗ Error: {}", e);
                
                // Show error chain if available
                let mut source = e.source();
                let mut level = 1;
                while let Some(err) = source {
                    println!("    Caused by ({}): {}", level, err);
                    source = err.source();
                    level += 1;
                }
            }
        }
    }
    
    println!();
}

/// Demo 10: Crossbeam - Advanced Concurrency
pub fn demo_10_crossbeam() {
    println!("=== Demo 10: Crossbeam ===");
    
    use crossbeam::channel::{unbounded, bounded, select, tick};
    use std::thread;
    use std::time::Duration;
    
    // Unbounded channel example
    println!("Unbounded channel communication:");
    let (tx, rx) = unbounded();
    
    // Producer thread
    let tx_clone = tx.clone();
    thread::spawn(move || {
        for i in 1..=5 {
            tx_clone.send(format!("Message {}", i)).unwrap();
            thread::sleep(Duration::from_millis(100));
        }
    });
    
    // Consumer
    for _ in 0..5 {
        if let Ok(msg) = rx.recv() {
            println!("  Received: {}", msg);
        }
    }
    
    // Bounded channel example
    println!("\nBounded channel (capacity 2):");
    let (tx_bounded, rx_bounded) = bounded(2);
    
    // This will demonstrate backpressure
    thread::spawn(move || {
        for i in 1..=5 {
            println!("  Sending message {}", i);
            tx_bounded.send(i).unwrap();
            println!("  Message {} sent", i);
        }
    });
    
    thread::sleep(Duration::from_millis(50));
    
    for _ in 0..5 {
        thread::sleep(Duration::from_millis(200)); // Slow consumer
        if let Ok(msg) = rx_bounded.recv() {
            println!("  Slowly received: {}", msg);
        }
    }
    
    // Select! macro example
    println!("\nChannel selection with select!:");
    let (tx1, rx1) = unbounded();
    let (tx2, rx2) = unbounded();
    let ticker = tick(Duration::from_millis(300));
    
    // Producers
    thread::spawn(move || {
        for i in 1..=3 {
            thread::sleep(Duration::from_millis(150));
            tx1.send(format!("Channel 1: {}", i)).unwrap();
        }
    });
    
    thread::spawn(move || {
        for i in 1..=3 {
            thread::sleep(Duration::from_millis(200));
            tx2.send(format!("Channel 2: {}", i)).unwrap();
        }
    });
    
    // Consumer with select
    let mut received = 0;
    while received < 6 {
        select! {
            recv(rx1) -> msg => {
                if let Ok(msg) = msg {
                    println!("  From rx1: {}", msg);
                    received += 1;
                }
            }
            recv(rx2) -> msg => {
                if let Ok(msg) = msg {
                    println!("  From rx2: {}", msg);
                    received += 1;
                }
            }
            recv(ticker) -> _ => {
                println!("  Tick! (periodic timer)");
            }
        }
    }
    
    println!();
}

/// Demo 11: Rayon - Data Parallelism
pub fn demo_11_rayon() {
    println!("=== Demo 11: Rayon ===");
    
    use rayon::prelude::*;
    use std::time::Instant;
    
    // Parallel iteration
    let numbers: Vec<i32> = (1..=1000).collect();
    
    // Sequential processing
    let start = Instant::now();
    let sequential_sum: i32 = numbers.iter().map(|&x| x * x).sum();
    let sequential_time = start.elapsed();
    
    // Parallel processing
    let start = Instant::now();
    let parallel_sum: i32 = numbers.par_iter().map(|&x| x * x).sum();
    let parallel_time = start.elapsed();
    
    println!("Sum of squares (1-1000):");
    println!("  Sequential: {} (took {:?})", sequential_sum, sequential_time);
    println!("  Parallel:   {} (took {:?})", parallel_sum, parallel_time);
    println!("  Speedup: {:.2}x", sequential_time.as_nanos() as f64 / parallel_time.as_nanos() as f64);
    
    // Parallel filtering and mapping
    let words = vec![
        "apple", "banana", "cherry", "date", "elderberry",
        "fig", "grape", "honeydew", "kiwi", "lemon",
    ];
    
    let long_words: Vec<String> = words
        .par_iter()
        .filter(|&&word| word.len() > 5)
        .map(|&word| word.to_uppercase())
        .collect();
    
    println!("\nParallel filter and map:");
    println!("  Long words (>5 chars): {:?}", long_words);
    
    // Parallel sorting
    let mut random_numbers: Vec<i32> = (1..=20).collect();
    random_numbers.reverse(); // Make it unsorted
    
    println!("\nParallel sorting:");
    println!("  Before: {:?}", &random_numbers[..10]);
    random_numbers.par_sort();
    println!("  After:  {:?}", &random_numbers[..10]);
    
    // Parallel reduce
    let text = "the quick brown fox jumps over the lazy dog";
    let word_count = text
        .par_split_whitespace()
        .map(|_| 1)
        .reduce(|| 0, |a, b| a + b);
    
    println!("\nParallel word counting:");
    println!("  Text: '{}'", text);
    println!("  Word count: {}", word_count);
    
    // Parallel find
    let target = 15;
    let found = numbers.par_iter().find_any(|&&x| x == target);
    println!("\nParallel find:");
    println!("  Looking for {}: {:?}", target, found);
    
    println!();
}

/// Demo 12: Tracing - Structured Logging
pub fn demo_12_tracing() {
    println!("=== Demo 12: Tracing ===");
    
    use tracing::{info, warn, error, debug, span, Level, instrument};
    use tracing_subscriber;
    
    // Initialize tracing subscriber
    let _ = tracing_subscriber::fmt()
        .with_max_level(Level::DEBUG)
        .try_init();
    
    #[instrument]
    fn process_order(order_id: u64, customer_id: u64) -> Result<String, &'static str> {
        info!(order_id, customer_id, "Processing order");
        
        // Simulate validation
        if order_id == 0 {
            error!("Invalid order ID");
            return Err("Invalid order ID");
        }
        
        // Create a span for payment processing
        let payment_span = span!(Level::INFO, "payment_processing", order_id);
        let _enter = payment_span.enter();
        
        info!("Processing payment");
        
        if customer_id == 999 {
            warn!(customer_id, "Customer flagged for review");
        }
        
        debug!("Payment validation complete");
        
        Ok(format!("Order {} processed successfully", order_id))
    }
    
    #[instrument(fields(user_id = %user_id))]
    fn user_login(user_id: u64, username: &str) {
        info!(username, "User attempting login");
        
        if username == "admin" {
            warn!("Admin login detected");
        }
        
        info!("Login successful");
    }
    
    println!("Demonstrating structured logging:");
    
    // Process some orders
    match process_order(12345, 67890) {
        Ok(result) => info!(result, "Order completed"),
        Err(e) => error!(error = e, "Order failed"),
    }
    
    match process_order(0, 12345) {
        Ok(result) => info!(result, "Order completed"),
        Err(e) => error!(error = e, "Order failed"),
    }
    
    match process_order(54321, 999) {
        Ok(result) => info!(result, "Order completed"),
        Err(e) => error!(error = e, "Order failed"),
    }
    
    // User login events
    user_login(123, "alice");
    user_login(456, "admin");
    
    println!();
}

/// Demo 13: Log + env_logger - Traditional Logging
pub fn demo_13_log() {
    println!("=== Demo 13: Log + env_logger ===");
    
    use log::{info, warn, error, debug};
    
    // Initialize env_logger
    let _ = env_logger::Builder::from_default_env()
        .filter_level(log::LevelFilter::Debug)
        .try_init();
    
    println!("Traditional logging examples:");
    
    info!("Application started");
    debug!("Debug information: connecting to database");
    
    // Simulate some application events
    for i in 1..=3 {
        info!("Processing item {}", i);
        
        if i == 2 {
            warn!("Item {} requires special handling", i);
        }
        
        debug!("Item {} processed successfully", i);
    }
    
    // Simulate an error condition
    error!("Failed to connect to external service");
    
    info!("Application shutting down");
    
    println!("Log messages sent to configured output");
    println!();
}

/// Demo 14: Itertools - Extended Iterator Methods
pub fn demo_14_itertools() {
    println!("=== Demo 14: Itertools ===");
    
    use itertools::Itertools;
    
    // Unique elements
    let numbers = vec![1, 2, 2, 3, 3, 3, 4, 4, 5];
    let unique: Vec<i32> = numbers.iter().cloned().unique().collect();
    println!("Unique elements:");
    println!("  Original: {:?}", numbers);
    println!("  Unique:   {:?}", unique);
    
    // Join strings
    let words = ["rust", "is", "awesome"];
    let joined = words.iter().join(" → ");
    println!("\nJoin operation:");
    println!("  Words: {:?}", words);
    println!("  Joined: {}", joined);
    
    // Group by
    let data = vec!["apple", "apricot", "banana", "blueberry", "cherry", "coconut"];
    println!("\nGroup by first letter:");
    for (letter, group) in &data.iter().group_by(|word| word.chars().next().unwrap()) {
        let items: Vec<&str> = group.cloned().collect();
        println!("  '{}': {:?}", letter, items);
    }
    
    // Chunks
    let numbers: Vec<i32> = (1..=10).collect();
    println!("\nChunks of 3:");
    for chunk in numbers.chunks(3) {
        println!("  {:?}", chunk);
    }
    
    // Cartesian product
    let colors = ["red", "green", "blue"];
    let sizes = ["S", "M", "L"];
    println!("\nCartesian product (colors × sizes):");
    for (color, size) in colors.iter().cartesian_product(sizes.iter()) {
        println!("  {}-{}", color, size);
    }
    
    // Combinations
    let items = ["A", "B", "C", "D"];
    println!("\nCombinations of 2:");
    for combo in items.iter().combinations(2) {
        println!("  {:?}", combo);
    }
    
    // Permutations
    let small_set = ["X", "Y", "Z"];
    println!("\nPermutations:");
    for perm in small_set.iter().permutations(2) {
        println!("  {:?}", perm);
    }
    
    // Intersperse
    let numbers = [1, 2, 3, 4, 5];
    let interspersed: Vec<i32> = itertools::Itertools::intersperse(numbers.iter().cloned(), 0).collect();
    println!("\nIntersperse with 0:");
    println!("  Original: {:?}", numbers);
    println!("  Interspersed: {:?}", interspersed);
    
    // Sorted and dedup
    let mixed = vec![3, 1, 4, 1, 5, 9, 2, 6, 5, 3];
    let sorted_unique: Vec<i32> = mixed.iter().cloned().sorted().dedup().collect();
    println!("\nSorted and deduplicated:");
    println!("  Original: {:?}", mixed);
    println!("  Processed: {:?}", sorted_unique);
    
    println!();
}

/// Demo 15: Once_cell - Lazy Static Initialization
pub fn demo_15_once_cell() {
    println!("=== Demo 15: Once_cell ===");
    
    use once_cell::sync::{Lazy, OnceCell};
    use std::collections::HashMap;
    
    // Lazy static initialization
    static CONFIG: Lazy<HashMap<&'static str, &'static str>> = Lazy::new(|| {
        println!("  Initializing global config (this happens only once)");
        let mut m = HashMap::new();
        m.insert("version", "1.0.0");
        m.insert("author", "Rust Demo");
        m.insert("debug", "true");
        m
    });
    
    static EXPENSIVE_COMPUTATION: Lazy<Vec<u64>> = Lazy::new(|| {
        println!("  Computing expensive result (this happens only once)");
        (1..=10).map(|x| x * x * x).collect() // Cubes
    });
    
    println!("Lazy static demonstration:");
    
    // First access - will trigger initialization
    println!("First access to CONFIG:");
    println!("  Version: {}", CONFIG.get("version").unwrap());
    
    // Second access - uses cached value
    println!("Second access to CONFIG:");
    println!("  Author: {}", CONFIG.get("author").unwrap());
    println!("  Debug: {}", CONFIG.get("debug").unwrap());
    
    // Expensive computation
    println!("Accessing expensive computation:");
    println!("  First access - cubes: {:?}", &EXPENSIVE_COMPUTATION[..5]);
    println!("  Second access - cubes: {:?}", &EXPENSIVE_COMPUTATION[5..]);
    
    // OnceCell for runtime initialization
    static RUNTIME_CONFIG: OnceCell<String> = OnceCell::new();
    
    fn get_runtime_config() -> &'static str {
        RUNTIME_CONFIG.get_or_init(|| {
            println!("  Initializing runtime config");
            format!("Runtime config initialized at startup")
        })
    }
    
    println!("Runtime initialization:");
    println!("  First call: {}", get_runtime_config());
    println!("  Second call: {}", get_runtime_config());
    
    println!();
}

/// Demo 16: UUID - Unique Identifier Generation
pub fn demo_16_uuid() {
    println!("=== Demo 16: UUID ===");
    
    use uuid::Uuid;
    
    // Generate different types of UUIDs
    println!("UUID generation:");
    
    // Version 4 (random)
    for i in 1..=3 {
        let id = Uuid::new_v4();
        println!("  Random UUID #{}: {}", i, id);
    }
    
    // Parse UUID from string
    let uuid_str = "550e8400-e29b-41d4-a716-446655440000";
    match Uuid::parse_str(uuid_str) {
        Ok(parsed) => {
            println!("\nParsed UUID: {}", parsed);
            println!("  Hyphenated: {}", parsed.hyphenated());
            println!("  Simple: {}", parsed.simple());
            println!("  URN: {}", parsed.urn());
        }
        Err(e) => println!("Failed to parse UUID: {}", e),
    }
    
    // UUID as bytes
    let uuid = Uuid::new_v4();
    let bytes = uuid.as_bytes();
    println!("\nUUID as bytes:");
    println!("  UUID: {}", uuid);
    println!("  Bytes: {:?}", bytes);
    println!("  From bytes: {}", Uuid::from_bytes(*bytes));
    
    // Nil UUID
    let nil = Uuid::nil();
    println!("\nSpecial UUIDs:");
    println!("  Nil UUID: {}", nil);
    println!("  Is nil: {}", nil.is_nil());
    
    // UUID for database keys
    println!("\nDatabase key simulation:");
    struct User {
        id: Uuid,
        name: String,
    }
    
    let users = vec![
        User { id: Uuid::new_v4(), name: "Alice".to_string() },
        User { id: Uuid::new_v4(), name: "Bob".to_string() },
        User { id: Uuid::new_v4(), name: "Charlie".to_string() },
    ];
    
    for user in users {
        println!("  User {}: {}", user.name, user.id);
    }
    
    println!();
}

/// Demo 17: Tempfile - Temporary File Management
pub fn demo_17_tempfile() {
    println!("=== Demo 17: Tempfile ===");
    
    use tempfile::{NamedTempFile, TempDir, Builder};
    use std::io::{Write, Read, Seek, SeekFrom};
    
    // Named temporary file
    println!("Named temporary file:");
    let mut temp_file = NamedTempFile::new().unwrap();
    
    // Write data
    writeln!(temp_file, "Hello, temporary world!").unwrap();
    writeln!(temp_file, "This file will be automatically deleted.").unwrap();
    writeln!(temp_file, "Line 3 of temporary data.").unwrap();
    
    println!("  Created temp file: {:?}", temp_file.path());
    
    // Read back the data
    temp_file.seek(SeekFrom::Start(0)).unwrap();
    let mut contents = String::new();
    temp_file.read_to_string(&mut contents).unwrap();
    
    println!("  File contents:");
    for (i, line) in contents.lines().enumerate() {
        println!("    Line {}: {}", i + 1, line);
    }
    
    // Temporary directory
    println!("\nTemporary directory:");
    let temp_dir = TempDir::new().unwrap();
    println!("  Created temp dir: {:?}", temp_dir.path());
    
    // Create files in the temp directory
    let file1_path = temp_dir.path().join("file1.txt");
    let file2_path = temp_dir.path().join("file2.txt");
    
    std::fs::write(&file1_path, "Content of file 1").unwrap();
    std::fs::write(&file2_path, "Content of file 2").unwrap();
    
    // List files in temp directory
    if let Ok(entries) = std::fs::read_dir(temp_dir.path()) {
        println!("  Files in temp directory:");
        for entry in entries {
            if let Ok(entry) = entry {
                let content = std::fs::read_to_string(entry.path()).unwrap_or_default();
                println!("    {:?}: {}", entry.file_name(), content);
            }
        }
    }
    
    println!("  Note: All temporary files/directories are automatically cleaned up when dropped");
    println!();
}

/// Demo 18: Bitflags - Type-safe Bit Flag Operations
pub fn demo_18_bitflags() {
    println!("=== Demo 18: Bitflags ===");
    
    use bitflags::bitflags;
    
    bitflags! {
        #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
        struct Permissions: u32 {
            const READ    = 0b00000001;
            const WRITE   = 0b00000010;
            const EXECUTE = 0b00000100;
            const DELETE  = 0b00001000;
            const ADMIN   = 0b10000000;
        }
    }
    
    println!("Permission system demonstration:");
    
    // Create permissions
    let user_perms = Permissions::READ | Permissions::WRITE;
    let admin_perms = Permissions::READ | Permissions::WRITE | Permissions::EXECUTE | Permissions::ADMIN;
    let readonly_perms = Permissions::READ;
    
    println!("  User permissions: {:?} (bits: {:08b})", user_perms, user_perms.bits());
    println!("  Admin permissions: {:?} (bits: {:08b})", admin_perms, admin_perms.bits());
    println!("  Readonly permissions: {:?} (bits: {:08b})", readonly_perms, readonly_perms.bits());
    
    // Check permissions
    println!("\nPermission checks:");
    println!("  User can read: {}", user_perms.contains(Permissions::READ));
    println!("  User can write: {}", user_perms.contains(Permissions::WRITE));
    println!("  User can execute: {}", user_perms.contains(Permissions::EXECUTE));
    println!("  User is admin: {}", user_perms.contains(Permissions::ADMIN));
    
    // Modify permissions
    let mut dynamic_perms = Permissions::READ;
    println!("\nDynamic permission changes:");
    println!("  Initial: {:?}", dynamic_perms);
    
    dynamic_perms.insert(Permissions::WRITE);
    println!("  After adding WRITE: {:?}", dynamic_perms);
    
    dynamic_perms.insert(Permissions::EXECUTE);
    println!("  After adding EXECUTE: {:?}", dynamic_perms);
    
    dynamic_perms.remove(Permissions::WRITE);
    println!("  After removing WRITE: {:?}", dynamic_perms);
    
    dynamic_perms.toggle(Permissions::DELETE);
    println!("  After toggling DELETE: {:?}", dynamic_perms);
    
    // Set operations
    println!("\nSet operations:");
    let perms1 = Permissions::READ | Permissions::WRITE;
    let perms2 = Permissions::WRITE | Permissions::EXECUTE;
    
    println!("  Perms1: {:?}", perms1);
    println!("  Perms2: {:?}", perms2);
    println!("  Union (|): {:?}", perms1 | perms2);
    println!("  Intersection (&): {:?}", perms1 & perms2);
    println!("  Difference (-): {:?}", perms1 - perms2);
    println!("  Symmetric difference (^): {:?}", perms1 ^ perms2);
    
    println!();
}

/// Demo 19: Parking_lot - High-performance Synchronization
pub fn demo_19_parking_lot() {
    println!("=== Demo 19: Parking_lot ===");
    
    use parking_lot::{Mutex, RwLock};
    use std::sync::Arc;
    use std::thread;
    use std::time::{Duration, Instant};
    
    // Basic mutex usage
    println!("High-performance mutex:");
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];
    
    let start = Instant::now();
    
    for i in 0..5 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            for _ in 0..1000 {
                let mut num = counter.lock();
                *num += 1;
            }
            println!("  Thread {} completed", i);
        });
        handles.push(handle);
    }
    
    for handle in handles {
        handle.join().unwrap();
    }
    
    let final_count = *counter.lock();
    let elapsed = start.elapsed();
    println!("  Final count: {} (took {:?})", final_count, elapsed);
    
    // RwLock for reader-writer scenarios
    println!("\nReader-writer lock:");
    let data = Arc::new(RwLock::new(vec![1, 2, 3, 4, 5]));
    let mut handles = vec![];
    
    // Multiple readers
    for i in 0..3 {
        let data = Arc::clone(&data);
        let handle = thread::spawn(move || {
            let reader = data.read();
            println!("  Reader {}: {:?}", i, *reader);
            thread::sleep(Duration::from_millis(100));
        });
        handles.push(handle);
    }
    
    // Single writer
    let data_writer = Arc::clone(&data);
    let writer_handle = thread::spawn(move || {
        thread::sleep(Duration::from_millis(50)); // Let readers start
        let mut writer = data_writer.write();
        writer.push(6);
        println!("  Writer: added element, new length: {}", writer.len());
    });
    handles.push(writer_handle);
    
    for handle in handles {
        handle.join().unwrap();
    }
    
    // Demonstrate try_lock
    println!("\nNon-blocking lock attempts:");
    let mutex = Arc::new(Mutex::new(42));
    let mutex_clone = Arc::clone(&mutex);
    
    let _guard = mutex.lock(); // Hold the lock
    
    thread::spawn(move || {
        match mutex_clone.try_lock() {
            Some(value) => println!("  Got lock: {}", *value),
            None => println!("  Lock was busy, couldn't acquire"),
        }
    }).join().unwrap();
    
    drop(_guard); // Release the lock
    
    match mutex.try_lock() {
        Some(value) => println!("  Now got lock: {}", *value),
        None => println!("  Still couldn't get lock"),
    }
    
    println!();
}

/// Demo 20: Advanced Collections Pattern (simulating dashmap)
pub fn demo_20_advanced_collections() {
    println!("=== Demo 20: Advanced Collections Pattern ===");
    
    use std::collections::{HashMap, BTreeMap, HashSet};
    use std::sync::{Arc, RwLock};
    use std::thread;
    
    // Concurrent HashMap pattern (like dashmap)
    println!("Concurrent HashMap pattern:");
    let concurrent_map = Arc::new(RwLock::new(HashMap::new()));
    let mut handles = vec![];
    
    for i in 0..3 {
        let map = Arc::clone(&concurrent_map);
        let handle = thread::spawn(move || {
            // Simulate concurrent writes
            {
                let mut map = map.write().unwrap();
                map.insert(format!("key_{}", i), i * 10);
                println!("  Thread {} inserted key_{} = {}", i, i, i * 10);
            }
            
            // Simulate concurrent reads
            {
                let map = map.read().unwrap();
                for (key, value) in map.iter() {
                    if key.contains(&i.to_string()) {
                        println!("  Thread {} read {}: {}", i, key, value);
                    }
                }
            }
        });
        handles.push(handle);
    }
    
    for handle in handles {
        handle.join().unwrap();
    }
    
    // Ordered map operations
    println!("\nOrdered collections:");
    let mut btree_map = BTreeMap::new();
    btree_map.insert("zebra", 1);
    btree_map.insert("apple", 2);
    btree_map.insert("banana", 3);
    btree_map.insert("cherry", 4);
    
    println!("  BTreeMap (sorted by key):");
    for (key, value) in &btree_map {
        println!("    {}: {}", key, value);
    }
    
    // Range queries
    println!("  Range query (a..=c):");
    for (key, value) in btree_map.range("a"..="c") {
        println!("    {}: {}", key, value);
    }
    
    // Set operations
    println!("\nSet operations:");
    let set1: HashSet<i32> = [1, 2, 3, 4, 5].iter().cloned().collect();
    let set2: HashSet<i32> = [4, 5, 6, 7, 8].iter().cloned().collect();
    
    println!("  Set 1: {:?}", set1);
    println!("  Set 2: {:?}", set2);
    
    let intersection: HashSet<_> = set1.intersection(&set2).cloned().collect();
    let union: HashSet<_> = set1.union(&set2).cloned().collect();
    let difference: HashSet<_> = set1.difference(&set2).cloned().collect();
    
    println!("  Intersection: {:?}", intersection);
    println!("  Union: {:?}", union);
    println!("  Difference (1-2): {:?}", difference);
    
    println!("\nNote: Real crates like dashmap provide production-ready");
    println!("concurrent collections with fine-grained locking.");
    
    println!();
}

/// Run all crate demonstrations
pub fn run_all_demos() {
    println!("🦀 RUST LECTURE - SECTION 8: POPULAR CRATE DEMONSTRATIONS 🦀");
    println!("==============================================================");
    println!();
    
    demo_1_serde_json();
    demo_2_rand();
    demo_3_clap();
    demo_4_tokio();
    demo_5_reqwest();
    demo_6_regex();
    demo_7_chrono();
    demo_8_anyhow();
    demo_9_thiserror();
    demo_10_crossbeam();
    demo_11_rayon();
    demo_12_tracing();
    demo_13_log();
    demo_14_itertools();
    demo_15_once_cell();
    demo_16_uuid();
    demo_17_tempfile();
    demo_18_bitflags();
    demo_19_parking_lot();
    demo_20_advanced_collections();
    
    println!("✅ Section 8 complete!");
    println!("💡 Key takeaway: Rust's crate ecosystem provides powerful, well-designed libraries for every need!");
}
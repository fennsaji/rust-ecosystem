// ===== ERROR HANDLING WITH CUSTOM TYPES =====
// 
// WHAT IS ERROR HANDLING?
// Error handling in Rust is explicit and type-safe. Instead of exceptions,
// Rust uses Result<T, E> types and provides excellent crates like thiserror
// and anyhow for ergonomic error handling.
//
// KEY CONCEPTS:
// • thiserror: Creates custom error types with derive macros
// • anyhow: Provides flexible error handling with context
// • Result<T, E>: Standard error return type
// • ? operator: Propagates errors up the call stack

use thiserror::Error;
use anyhow::{Context, Result as AnyhowResult};
use std::fs;
use std::io;

// ===== 1. CUSTOM ERROR TYPES WITH THISERROR =====
//
// DEFINING A CUSTOM ERROR ENUM:
// thiserror provides derive macros to automatically implement
// std::error::Error trait and Display formatting
#[derive(Error, Debug)]
pub enum TaskError {
    // IO errors with context
    #[error("Failed to read file: {path}")]
    FileReadError { path: String },
    
    // Validation errors
    #[error("Invalid input: {message}")]
    ValidationError { message: String },
    
    // Network errors
    #[error("Network timeout after {seconds} seconds")]
    NetworkTimeout { seconds: u64 },
    
    // Wrapping other error types
    #[error("IO operation failed")]
    IoError(#[from] io::Error),
    
    // Parse errors
    #[error("Failed to parse number: {input}")]
    ParseError { input: String },
    
    // Configuration errors
    #[error("Configuration missing: {key}")]
    ConfigError { key: String },
}

// ===== 2. FUNCTIONS RETURNING CUSTOM ERRORS =====
//
// FUNCTION THAT CAN FAIL WITH CUSTOM ERROR:
// Using Result<T, TaskError> for explicit error handling
pub fn read_config_file(path: &str) -> Result<String, TaskError> {
    // EXAMPLE OF CONVERTING IO ERROR TO CUSTOM ERROR:
    // The #[from] attribute automatically converts io::Error to TaskError::IoError
    let content = fs::read_to_string(path)
        .map_err(|_| TaskError::FileReadError {
            path: path.to_string(),
        })?;
    
    // VALIDATION WITH CUSTOM ERROR:
    if content.is_empty() {
        return Err(TaskError::ValidationError {
            message: "Configuration file is empty".to_string(),
        });
    }
    
    Ok(content)
}

// ===== 3. PARSING WITH ERROR HANDLING =====
//
// FUNCTION THAT PARSES AND HANDLES ERRORS:
pub fn parse_number(input: &str) -> Result<i32, TaskError> {
    // VALIDATION FIRST:
    if input.is_empty() {
        return Err(TaskError::ValidationError {
            message: "Input cannot be empty".to_string(),
        });
    }
    
    // PARSING WITH CUSTOM ERROR:
    input.parse::<i32>()
        .map_err(|_| TaskError::ParseError {
            input: input.to_string(),
        })
}

// ===== 4. USING ANYHOW FOR FLEXIBLE ERROR HANDLING =====
//
// ANYHOW PROVIDES CONTEXT AND FLEXIBLE ERROR HANDLING:
// Good for applications where you need to handle many different error types
pub fn process_user_data(user_id: u32) -> AnyhowResult<String> {
    // ADDING CONTEXT TO ERRORS:
    let config = read_config_file("config.toml")
        .with_context(|| format!("Failed to load config for user {}", user_id))?;
    
    // CHAINING OPERATIONS WITH CONTEXT:
    let lines: Vec<&str> = config.lines().collect();
    let first_line = lines.first()
        .ok_or_else(|| anyhow::anyhow!("Config file has no content"))?;
    
    // PARSING WITH CONTEXT:
    let setting = first_line.split('=').nth(1)
        .ok_or_else(|| anyhow::anyhow!("Invalid config format"))?;
    
    Ok(setting.trim().to_string())
}

// ===== 5. NETWORK SIMULATION WITH TIMEOUT =====
//
// SIMULATING NETWORK OPERATIONS WITH CUSTOM TIMEOUTS:
pub async fn fetch_data_with_timeout(url: &str, timeout_seconds: u64) -> Result<String, TaskError> {
    // SIMULATE NETWORK DELAY:
    tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
    
    // SIMULATE TIMEOUT ERROR:
    if timeout_seconds < 1 {
        return Err(TaskError::NetworkTimeout {
            seconds: timeout_seconds,
        });
    }
    
    // SIMULATE SUCCESSFUL RESPONSE:
    Ok(format!("Data from {}", url))
}

// ===== 6. VALIDATION WITH MULTIPLE CHECKS =====
//
// FUNCTION WITH MULTIPLE VALIDATION STEPS:
pub fn validate_user_input(name: &str, email: &str, age: &str) -> Result<(String, String, u32), TaskError> {
    // NAME VALIDATION:
    if name.is_empty() {
        return Err(TaskError::ValidationError {
            message: "Name cannot be empty".to_string(),
        });
    }
    
    // EMAIL VALIDATION:
    if !email.contains('@') {
        return Err(TaskError::ValidationError {
            message: "Invalid email format".to_string(),
        });
    }
    
    // AGE PARSING AND VALIDATION:
    let age_num = parse_number(age)?;
    if age_num < 0 || age_num > 150 {
        return Err(TaskError::ValidationError {
            message: "Age must be between 0 and 150".to_string(),
        });
    }
    
    Ok((name.to_string(), email.to_string(), age_num as u32))
}

// ===== 7. COMBINING MULTIPLE ERROR SOURCES =====
//
// FUNCTION THAT COMBINES MULTIPLE ERROR SOURCES:
pub fn complex_operation(path: &str, number_str: &str) -> AnyhowResult<i32> {
    // FILE OPERATION:
    let content = read_config_file(path)
        .with_context(|| "Failed during file reading phase")?;
    
    // NUMBER PARSING:
    let base_number = parse_number(number_str)
        .with_context(|| "Failed during number parsing phase")?;
    
    // ADDITIONAL PROCESSING:
    let result = base_number * content.lines().count() as i32;
    
    // VALIDATION OF RESULT:
    if result > 1000 {
        return Err(anyhow::anyhow!("Result too large: {}", result));
    }
    
    Ok(result)
}

// ===== 8. DEMONSTRATION FUNCTIONS =====
//
// FUNCTION TO DEMONSTRATE ERROR HANDLING:
pub fn demonstrate_error_handling() {
    println!("=== ERROR HANDLING DEMONSTRATIONS ===");
    
    // TESTING SUCCESSFUL OPERATIONS:
    println!("\n1. Testing successful operations:");
    match parse_number("42") {
        Ok(num) => println!("   ✓ Parsed number: {}", num),
        Err(e) => println!("   ✗ Error: {}", e),
    }
    
    // TESTING VALIDATION ERRORS:
    println!("\n2. Testing validation errors:");
    match validate_user_input("", "test@example.com", "25") {
        Ok((name, email, age)) => println!("   ✓ Valid user: {} ({}) age {}", name, email, age),
        Err(e) => println!("   ✗ Validation error: {}", e),
    }
    
    // TESTING PARSE ERRORS:
    println!("\n3. Testing parse errors:");
    match parse_number("not_a_number") {
        Ok(num) => println!("   ✓ Parsed: {}", num),
        Err(e) => println!("   ✗ Parse error: {}", e),
    }
    
    // TESTING FILE ERRORS:
    println!("\n4. Testing file errors:");
    match read_config_file("nonexistent.toml") {
        Ok(content) => println!("   ✓ File content: {}", content),
        Err(e) => println!("   ✗ File error: {}", e),
    }
}

// ===== 9. ASYNC ERROR HANDLING =====
//
// ASYNC FUNCTION WITH ERROR HANDLING:
pub async fn demonstrate_async_errors() {
    println!("\n=== ASYNC ERROR HANDLING ===");
    
    // TESTING TIMEOUT ERROR:
    println!("\n1. Testing network timeout:");
    match fetch_data_with_timeout("https://example.com", 0).await {
        Ok(data) => println!("   ✓ Fetched: {}", data),
        Err(e) => println!("   ✗ Network error: {}", e),
    }
    
    // TESTING SUCCESSFUL FETCH:
    println!("\n2. Testing successful fetch:");
    match fetch_data_with_timeout("https://example.com", 5).await {
        Ok(data) => println!("   ✓ Fetched: {}", data),
        Err(e) => println!("   ✗ Network error: {}", e),
    }
}

// ===== 10. ERROR PROPAGATION PATTERNS =====
//
// FUNCTION SHOWING ERROR PROPAGATION WITH ? OPERATOR:
pub fn propagate_errors(input: &str) -> Result<i32, TaskError> {
    // THE ? OPERATOR AUTOMATICALLY PROPAGATES ERRORS:
    // If parse_number returns Err, this function returns that error
    // If parse_number returns Ok(value), execution continues
    let number = parse_number(input)?;
    
    // ADDITIONAL VALIDATION:
    if number < 0 {
        return Err(TaskError::ValidationError {
            message: "Number must be positive".to_string(),
        });
    }
    
    Ok(number * 2)
}

// ===== KEY TAKEAWAYS =====
//
// ERROR HANDLING BEST PRACTICES:
// 1. Use thiserror for custom error types in libraries
// 2. Use anyhow for flexible error handling in applications
// 3. Prefer Result<T, E> over unwrap() or expect()
// 4. Use the ? operator for clean error propagation
// 5. Add context to errors for better debugging
// 6. Design error types to be informative and actionable
// 7. Consider using From trait for error conversions
// 8. Test both success and error cases
//
// WHEN TO USE EACH:
// • thiserror: When you need structured, specific error types
// • anyhow: When you need flexible error handling across different error types
// • Result<T, E>: Always prefer over exceptions or panics
// • ? operator: For clean error propagation without nested match statements
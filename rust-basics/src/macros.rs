// ===== DECLARATIVE MACROS DEEP STUDY =====
// 
// WHAT ARE DECLARATIVE MACROS?
// Declarative macros (macro_rules!) are a way to write code that writes code.
// They work by pattern matching on the syntax of the code passed to them
// and generating new code based on those patterns.
//
// KEY CONCEPTS:
// • Macros are expanded at compile time
// • They operate on token trees, not values
// • Pattern matching with different syntax forms
// • Hygiene prevents variable name collisions
// • Can generate repetitive code efficiently

use colored::*;

// ===== 1. BASIC LOG MACRO =====
//
// CORE LOG MACRO WITH PATTERN MATCHING:
// This macro takes a log level and message, formats them with colors
// and prints them with timestamp and location information
macro_rules! log {
    // PATTERN 1: Simple message string
    ($level:ident, $message:expr) => {
        {
            let timestamp = chrono::Local::now().format("%Y-%m-%d %H:%M:%S");
            let file = file!();
            let line = line!();
            
            // MATCH LOG LEVEL AND APPLY COLORS:
            let (level_str, color_fn): (&str, fn(&str) -> ColoredString) = match stringify!($level) {
                "info" => ("INFO", |s| s.blue()),
                "warn" => ("WARN", |s| s.yellow()),
                "error" => ("ERROR", |s| s.red()),
                "debug" => ("DEBUG", |s| s.cyan()),
                "trace" => ("TRACE", |s| s.magenta()),
                _ => ("LOG", |s| s.white()),
            };
            
            // PRINT FORMATTED LOG MESSAGE:
            println!("[{}] {} {} - {} ({}:{})", 
                timestamp.to_string().dimmed(),
                color_fn(&format!("[{}]", level_str)),
                $message,
                "rust-basics".green(),
                file,
                line
            );
        }
    };
    
    // PATTERN 2: Formatted message with arguments
    ($level:ident, $format:expr, $($arg:expr),*) => {
        log!($level, format!($format, $($arg),*))
    };
    
    // PATTERN 3: Message with additional context
    ($level:ident, $message:expr, context: $context:expr) => {
        {
            let timestamp = chrono::Local::now().format("%Y-%m-%d %H:%M:%S");
            let file = file!();
            let line = line!();
            
            let (level_str, color_fn): (&str, fn(&str) -> ColoredString) = match stringify!($level) {
                "info" => ("INFO", |s| s.blue()),
                "warn" => ("WARN", |s| s.yellow()),
                "error" => ("ERROR", |s| s.red()),
                "debug" => ("DEBUG", |s| s.cyan()),
                "trace" => ("TRACE", |s| s.magenta()),
                _ => ("LOG", |s| s.white()),
            };
            
            println!("[{}] {} {} | {} - {} ({}:{})", 
                timestamp.to_string().dimmed(),
                color_fn(&format!("[{}]", level_str)),
                $message,
                format!("Context: {}", $context).italic(),
                "rust-basics".green(),
                file,
                line
            );
        }
    };
}

// ===== 2. CONVENIENCE MACROS =====
//
// SPECIFIC LOG LEVEL MACROS:
// These macros provide convenient shortcuts for common log levels
// They all delegate to the main log! macro

// INFO MACRO - for general information
macro_rules! info {
    ($message:expr) => {
        log!(info, $message)
    };
    ($format:expr, $($arg:expr),*) => {
        log!(info, $format, $($arg),*)
    };
    ($message:expr, context: $context:expr) => {
        log!(info, $message, context: $context)
    };
}

// ERROR MACRO - for error messages
macro_rules! error {
    ($message:expr) => {
        log!(error, $message)
    };
    ($format:expr, $($arg:expr),*) => {
        log!(error, $format, $($arg),*)
    };
    ($message:expr, context: $context:expr) => {
        log!(error, $message, context: $context)
    };
}

// WARN MACRO - for warnings
macro_rules! warn {
    ($message:expr) => {
        log!(warn, $message)
    };
    ($format:expr, $($arg:expr),*) => {
        log!(warn, $format, $($arg),*)
    };
    ($message:expr, context: $context:expr) => {
        log!(warn, $message, context: $context)
    };
}

// DEBUG MACRO - for debug information
macro_rules! debug {
    ($message:expr) => {
        log!(debug, $message)
    };
    ($format:expr, $($arg:expr),*) => {
        log!(debug, $format, $($arg),*)
    };
    ($message:expr, context: $context:expr) => {
        log!(debug, $message, context: $context)
    };
}

// ===== 3. ADVANCED MACRO PATTERNS =====
//
// MACRO WITH REPETITION:
// This macro can take multiple key-value pairs and format them
macro_rules! log_with_fields {
    ($level:ident, $message:expr, $($key:ident = $value:expr),*) => {
        {
            let mut fields = Vec::new();
            $(
                fields.push(format!("{}={}", stringify!($key), $value));
            )*
            
            let fields_str = fields.join(", ");
            log!($level, format!("{} [{}]", $message, fields_str));
        }
    };
}

// MACRO FOR TIMING OPERATIONS:
// This macro measures execution time of a block of code
macro_rules! time_operation {
    ($name:expr, $block:block) => {
        {
            let start = std::time::Instant::now();
            info!("Starting operation: {}", $name);
            
            let result = $block;
            
            let duration = start.elapsed();
            info!("Operation '{}' completed in {:?}", $name, duration);
            
            result
        }
    };
}

// MACRO FOR CONDITIONAL LOGGING:
// This macro only logs if a condition is true
macro_rules! log_if {
    ($condition:expr, $level:ident, $message:expr) => {
        if $condition {
            log!($level, $message);
        }
    };
    ($condition:expr, $level:ident, $format:expr, $($arg:expr),*) => {
        if $condition {
            log!($level, $format, $($arg),*);
        }
    };
}

// ===== 4. MACRO FOR DEBUGGING VALUES =====
//
// DBGLOG MACRO - prints variable names and values
macro_rules! dbglog {
    ($($val:expr),*) => {
        {
            print!("{} ", "[DEBUG]".cyan());
            $(
                print!("{} = {:?} ", stringify!($val), $val);
            )*
            println!();
        }
    };
}

// ===== 5. MACRO WITH OPTIONAL PARAMETERS =====
//
// FLEXIBLE LOG MACRO WITH OPTIONAL TIMESTAMP
macro_rules! flexible_log {
    // Without timestamp
    (no_time, $level:ident, $message:expr) => {
        {
            let (level_str, color_fn): (&str, fn(&str) -> ColoredString) = match stringify!($level) {
                "info" => ("INFO", |s| s.blue()),
                "warn" => ("WARN", |s| s.yellow()),
                "error" => ("ERROR", |s| s.red()),
                _ => ("LOG", |s| s.white()),
            };
            
            println!("{} {}", color_fn(&format!("[{}]", level_str)), $message);
        }
    };
    
    // With timestamp (default behavior)
    ($level:ident, $message:expr) => {
        log!($level, $message)
    };
}

// ===== 6. DEMONSTRATION FUNCTIONS =====
//
// FUNCTION TO DEMONSTRATE BASIC MACRO USAGE:
pub fn demonstrate_basic_macros() {
    println!("=== BASIC MACRO DEMONSTRATIONS ===");
    
    // BASIC LOG MESSAGES:
    info!("This is an info message");
    warn!("This is a warning message");
    error!("This is an error message");
    debug!("This is a debug message");
    
    // FORMATTED MESSAGES:
    info!("User {} logged in with ID {}", "Alice", 12345);
    warn!("Memory usage at {}%", 85);
    error!("Failed to connect to database after {} attempts", 3);
    
    // MESSAGES WITH CONTEXT:
    info!("Processing user request", context: "user_service");
    error!("Database connection failed", context: "connection_pool");
}

// FUNCTION TO DEMONSTRATE ADVANCED MACROS:
pub fn demonstrate_advanced_macros() {
    println!("\n=== ADVANCED MACRO DEMONSTRATIONS ===");
    
    // MACRO WITH FIELDS:
    log_with_fields!(info, "User action", user_id = 123, action = "login", ip = "192.168.1.1");
    log_with_fields!(error, "Transaction failed", amount = 100.50, account = "ACC001", error_code = 500);
    
    // TIMING MACRO:
    let result = time_operation!("Database Query", {
        // Simulate some work
        std::thread::sleep(std::time::Duration::from_millis(100));
        "Query result"
    });
    info!("Query returned: {}", result);
    
    // CONDITIONAL LOGGING:
    let debug_mode = true;
    let user_count = 42;
    
    log_if!(debug_mode, debug, "Debug mode is enabled");
    log_if!(user_count > 50, warn, "High user count: {}", user_count);
    log_if!(user_count > 100, error, "Critical user count: {}", user_count);
    
    // DEBUGGING VALUES:
    let x = 10;
    let y = 20;
    let name = "Rust";
    dbglog!(x, y, name, x + y);
    
    // FLEXIBLE LOGGING:
    flexible_log!(info, "This has a timestamp");
    flexible_log!(no_time, warn, "This has no timestamp");
}

// ===== 7. MACRO EXPANSION EXAMPLES =====
//
// FUNCTION TO SHOW WHAT MACROS EXPAND TO:
pub fn demonstrate_macro_expansion() {
    println!("\n=== MACRO EXPANSION EXAMPLES ===");
    
    // SIMPLE EXPANSION:
    info!("Simple message");
    
    // FORMATTED EXPANSION:
    info!("Formatted message: {}", "Hello World");
    
    // CONTEXT EXPANSION:
    info!("Message with context", context: "demo");
    
    // TIMING EXPANSION:
    time_operation!("Simple calculation", {
        let result = 2 + 2;
        result
    });
}

// ===== 8. MACRO BEST PRACTICES EXAMPLES =====
//
// FUNCTION DEMONSTRATING MACRO BEST PRACTICES:
pub fn demonstrate_macro_best_practices() {
    println!("\n=== MACRO BEST PRACTICES ===");
    
    // 1. CLEAR NAMING:
    info!("Use descriptive names for macros");
    
    // 2. CONSISTENT PATTERNS:
    info!("Basic message");
    info!("Formatted message: {}", "with args");
    
    // 3. PROPER ERROR HANDLING:
    error!("Always handle errors gracefully");
    
    // 4. DOCUMENTATION:
    debug!("Document your macros well");
    
    // 5. TESTING:
    info!("Test macro expansion thoroughly");
}

// ===== 9. MACRO HYGIENE DEMONSTRATION =====
//
// MACRO THAT DEMONSTRATES HYGIENE:
macro_rules! hygienic_macro {
    () => {
        {
            let x = "macro variable";
            println!("Inside macro: {}", x);
            x
        }
    };
}

pub fn demonstrate_macro_hygiene() {
    println!("\n=== MACRO HYGIENE DEMONSTRATION ===");
    
    let x = "function variable";
    println!("Before macro: {}", x);
    
    let macro_result = hygienic_macro!();
    println!("After macro: {}", x);
    println!("Macro returned: {}", macro_result);
    
    info!("Macro variables don't interfere with surrounding scope");
}

// ===== 10. MAIN DEMONSTRATION FUNCTION =====
//
// FUNCTION TO RUN ALL DEMONSTRATIONS:
pub fn demonstrate_macros() {
    println!("🦀 RUST DECLARATIVE MACROS DEEP STUDY 🦀");
    
    demonstrate_basic_macros();
    demonstrate_advanced_macros();
    demonstrate_macro_expansion();
    demonstrate_macro_best_practices();
    demonstrate_macro_hygiene();
    
    println!("\n🎯 MACRO CONCEPTS SUMMARY:");
    println!("✅ Declarative macros use pattern matching");
    println!("✅ Macros expand at compile time");
    println!("✅ Multiple patterns can be matched");
    println!("✅ Repetition patterns with $()*");
    println!("✅ Hygiene prevents variable conflicts");
    println!("✅ Built-in macros: file!(), line!(), stringify!()");
    
    println!("\n📊 MACRO USAGE PATTERNS:");
    println!("• Code generation and repetition");
    println!("• Domain-specific languages (DSLs)");
    println!("• Configuration and setup");
    println!("• Logging and debugging");
    println!("• Testing utilities");
    
    println!("\n💡 MACRO BEST PRACTICES:");
    println!("• Keep macros simple and focused");
    println!("• Use descriptive names");
    println!("• Document macro behavior");
    println!("• Test macro expansion");
    println!("• Prefer functions when possible");
    println!("• Use cargo expand to debug");
}

// ===== KEY TAKEAWAYS =====
//
// DECLARATIVE MACRO CONCEPTS:
// 1. Pattern matching on token trees
// 2. Compile-time code generation
// 3. Hygiene prevents variable capture
// 4. Multiple pattern arms for flexibility
// 5. Repetition with $()*
// 6. Built-in macros for meta-information
//
// WHEN TO USE MACROS:
// • Reducing code duplication
// • Creating domain-specific syntax
// • Compile-time computation
// • Code generation based on patterns
// • When functions aren't sufficient
//
// MACRO DEBUGGING:
// • Use cargo expand to see generated code
// • Add println! statements in macro arms
// • Test with simple inputs first
// • Use rust-analyzer for macro highlighting
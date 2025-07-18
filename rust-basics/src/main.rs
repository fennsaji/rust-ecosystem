// ===== RUST ECOSYSTEM LEARNING PROJECT =====
// 
// This project is organized into modules for learning different Rust concepts:
// - traits: Deep study of Rust's trait system
// - lifetimes: Deep study of Rust's lifetime system
// - smart_pointers: Deep study of Rust's smart pointers (Box, Rc, RefCell, Arc)
// - errors: Deep study of error handling with thiserror and anyhow
// - macros: Deep study of declarative macros (macro_rules!)
// - (future modules will be added here)

mod traits;
mod lifetimes;
mod smart_pointers;
mod errors;
mod macros;

#[tokio::main]
async fn main() {
    println!("🦀 RUST ECOSYSTEM LEARNING PROJECT 🦀\n");
    
    // Module 1: Traits System Deep Study
    println!("📚 MODULE 1: TRAIT SYSTEM DEEP STUDY");
    println!("=====================================");
    traits::demonstrate_traits();
    
    println!("\n\n");
    
    // Module 2: Lifetimes Deep Study
    println!("📚 MODULE 2: LIFETIMES DEEP STUDY");
    println!("==================================");
    lifetimes::demonstrate_lifetimes();
    
    println!("\n\n");
    
    // Module 3: Smart Pointers Deep Study
    println!("📚 MODULE 3: SMART POINTERS DEEP STUDY");
    println!("=======================================");
    smart_pointers::demonstrate_smart_pointers();
    
    println!("\n\n");
    
    // Module 4: Error Handling Deep Study
    println!("📚 MODULE 4: ERROR HANDLING DEEP STUDY");
    println!("=======================================");
    errors::demonstrate_error_handling();
    errors::demonstrate_async_errors().await;
    
    println!("\n\n");
    
    // Module 5: Declarative Macros Deep Study
    println!("📚 MODULE 5: DECLARATIVE MACROS DEEP STUDY");
    println!("===========================================");
    macros::demonstrate_macros();
    
    println!("\n\n🎯 LEARNING PROGRESS:");
    println!("✅ Module 1: Trait System (traits.rs)");
    println!("✅ Module 2: Lifetimes (lifetimes.rs)");
    println!("✅ Module 3: Smart Pointers (smart_pointers.rs)");
    println!("✅ Module 4: Error Handling (errors.rs)");
    println!("✅ Module 5: Declarative Macros (macros.rs)");
    println!("⏳ More modules coming soon...");
}

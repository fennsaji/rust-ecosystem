// ===== TRAIT SYSTEM DEEP STUDY =====
// 
// WHAT ARE TRAITS?
// Traits define shared behavior that types can implement. They're similar to interfaces
// in other languages but more powerful. Traits allow you to define method signatures
// that types must implement, enabling polymorphism and code reuse.

use std::fmt::Display;

// ===== 1. BASIC TRAIT DEFINITION =====
// 
// A trait defines a set of methods that implementors must provide.
// Methods can have default implementations or be required (no body).
pub trait Printable {
    // Required method - implementors MUST provide this
    fn print(&self);
    
    // Default implementation - implementors CAN override this
    fn print_twice(&self) {
        self.print();
        self.print();
    }
    
    // Associated function (no self) - like a static method
    // 
    // UNDERSTANDING: &'static str where Self: Sized
    // ==============================================
    // 
    // Breaking down the syntax:
    // • &'static str: Return type is a string slice with 'static lifetime
    //   - 'static means the string lives for the entire program duration
    //   - Common for string literals like "Article" baked into the binary
    // 
    // • where Self: Sized: Trait bound constraint
    //   - Self: Refers to the concrete type implementing the trait
    //   - Sized: Marker trait indicating the type has known size at compile time
    //   - where: Introduces a trait bound constraint
    // 
    // WHY THIS CONSTRAINT EXISTS:
    // • For trait objects (dyn Trait) to work, traits must be "object-safe"
    // • Object-safe traits cannot have:
    //   - Associated functions without self parameter
    //   - Methods that use Self in unsupported ways
    //   - Generic methods
    // 
    // HOW "where Self: Sized" FIXES IT:
    // • ✅ Static dispatch works: Article::type_name() ✓
    // • ✅ Dynamic dispatch works: &dyn Printable ✓ (method excluded from vtable)
    // • ❌ But: Can't call type_name() on trait objects
    // 
    // PRACTICAL EXAMPLE:
    // let article = Article { /* ... */ };
    // println!("{}", Article::type_name()); // ✅ Works - "Article"
    // 
    // let printable: &dyn Printable = &article;
    // printable.print(); // ✅ Works - other methods available
    // printable.type_name(); // ❌ Compile error - not available on trait objects
    // 
    // ALTERNATIVE APPROACHES:
    // 1. Make it a method: fn type_name(&self) -> &'static str
    // 2. Separate trait: trait TypeName { fn type_name() -> &'static str; }
    // 
    // The constraint says: "This method is only available for concrete types, not trait objects"
    fn type_name() -> &'static str where Self: Sized {
        "Unknown"
    }
}

// ===== 2. SUMMARIZABLE TRAIT WITH GENERICS =====
//
// This trait demonstrates how to use generics within trait definitions
// and how to specify trait bounds on generic parameters.
pub trait Summarizable<T> {
    fn summarize(&self) -> T;
    
    // Method with trait bounds - T must implement Display
fn summarize_with_context(&self, context: &str) -> String 
    where 
        T: Display,
    {
        format!("{}: {}", context, self.summarize())
    }
}

// ===== 3. EXAMPLE TYPES TO IMPLEMENT TRAITS ON =====

#[derive(Debug, Clone)]
pub struct Article {
    pub title: String,
    pub content: String,
    pub author: String,
}

#[derive(Debug, Clone)]
pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply_to: Option<String>,
}

#[derive(Debug)]
pub struct Book {
    pub title: String,
    pub author: String,
    pub pages: u32,
}

// ===== 4. TRAIT IMPLEMENTATIONS =====

// Implementing Printable for Article
impl Printable for Article {
    fn print(&self) {
        println!("📰 Article: '{}' by {}", self.title, self.author);
    }
    
    // Override the default implementation
    fn print_twice(&self) {
        println!("🔄 Printing article twice:");
        self.print();
        println!("   (second time)");
        self.print();
    }
    
    fn type_name() -> &'static str {
        "Article"
    }
}

// Implementing Printable for Tweet
impl Printable for Tweet {
    fn print(&self) {
        match &self.reply_to {
            Some(reply) => println!("🐦 @{} (replying to {}): {}", self.username, reply, self.content),
            None => println!("🐦 @{}: {}", self.username, self.content),
        }
    }
    
    fn type_name() -> &'static str {
        "Tweet"
    }
}

// Implementing Printable for Book
impl Printable for Book {
    fn print(&self) {
        println!("📚 Book: '{}' by {} ({} pages)", self.title, self.author, self.pages);
    }
    
    fn type_name() -> &'static str {
        "Book"
    }
}

// ===== 5. SUMMARIZABLE IMPLEMENTATIONS =====

impl Summarizable<String> for Article {
    fn summarize(&self) -> String {
        format!("{} by {}", self.title, self.author)
    }
}

impl Summarizable<String> for Tweet {
    fn summarize(&self) -> String {
        format!("@{}: {}", self.username, 
                if self.content.len() > 50 {
                    format!("{}...", &self.content[..50])
                } else {
                    self.content.clone()
                })
    }
}

impl Summarizable<u32> for Book {
    fn summarize(&self) -> u32 {
        self.pages
    }
}

// ===== 6. FUNCTIONS WITH TRAIT BOUNDS =====

// UNDERSTANDING DIFFERENT TRAIT USAGE PATTERNS:
// ==============================================
// 
// 1. GENERIC WITH TRAIT BOUNDS: <T: Trait>
//    • When: Need compile-time polymorphism, performance critical
//    • How: Monomorphization creates separate code for each type
//    • Pro: Zero-cost abstraction, inlined, fast
//    • Con: Code bloat, compile-time only
//    • Use: Libraries, performance-critical code, when types known at compile time
// 
// 2. IMPL TRAIT: impl Trait
//    • When: Cleaner syntax for simple cases, return types
//    • How: Syntactic sugar for generic bounds
//    • Pro: Readable, same performance as generics
//    • Con: Less flexible than full generics
//    • Use: Function parameters, return types, when you don't need the full power of generics
// 
// 3. TRAIT OBJECTS: &dyn Trait
//    • When: Runtime polymorphism, heterogeneous collections
//    • How: Virtual table (vtable) lookup at runtime
//    • Pro: Flexible, can store different types together
//    • Con: Runtime overhead, heap allocation sometimes needed
//    • Use: Plugin systems, GUI event handlers, when types unknown at compile time
// 
// 4. BOXED TRAIT OBJECTS: Box<dyn Trait>
//    • When: Owned trait objects, recursive types, return different types
//    • How: Heap allocation + vtable lookup
//    • Pro: Ownership, can return from functions, recursive structures
//    • Con: Heap allocation cost, runtime dispatch overhead
//    • Use: Factory patterns, recursive data structures, when returning trait objects

// Function that accepts any type implementing Printable
// USES: <T: Printable> - Generic with trait bounds
// REASON: Compile-time optimization, zero-cost abstraction
pub fn print_item<T: Printable>(item: &T) {
    println!("🔧 Generic print function:");
    item.print();
}

// Function with multiple trait bounds
// USES: <T> where T: Multiple + Bounds - Complex trait bounds
// REASON: Multiple constraints, cleaner syntax with where clause
pub fn print_and_summarize<T>(item: &T) 
where 
    T: Printable + Summarizable<String>,
{
    println!("🔧 Print and summarize:");
    item.print();
    println!("   Summary: {}", item.summarize());
}

// Function using impl Trait syntax (syntactic sugar for trait bounds)
// USES: &impl Trait - Impl trait syntax
// REASON: Cleaner syntax for simple cases, same performance as generics
pub fn print_with_impl_trait(item: &impl Printable) {
    println!("🔧 Using impl Trait syntax:");
    item.print();
}

// ===== 7. DYNAMIC DISPATCH WITH TRAIT OBJECTS =====

// Function that accepts a trait object - enables runtime polymorphism
// USES: &dyn Trait - Borrowed trait object
// REASON: Runtime polymorphism, can accept different types at runtime
pub fn print_dynamic(item: &dyn Printable) {
    println!("🎭 Dynamic dispatch:");
    item.print();
}

// Function that returns a trait object
// USES: Box<dyn Trait> - Owned trait object
// REASON: Need to return owned trait object, different types based on runtime condition
pub fn create_printable_item(choice: u8) -> Box<dyn Printable> {
    match choice {
        1 => Box::new(Article {
            title: "Dynamic Dispatch in Rust".to_string(),
            content: "Trait objects enable runtime polymorphism...".to_string(),
            author: "Rust Expert".to_string(),
        }),
        2 => Box::new(Tweet {
            username: "rustacean".to_string(),
            content: "Trait objects are powerful! 🦀".to_string(),
            reply_to: None,
        }),
        _ => Box::new(Book {
            title: "The Rust Programming Language".to_string(),
            author: "Steve Klabnik".to_string(),
            pages: 560,
        }),
    }
}

// ===== 8. ADVANCED TRAIT FEATURES =====

// Trait with associated types (more advanced than generics)
pub trait Iterator {
    type Item;  // Associated type
    
    fn next(&mut self) -> Option<Self::Item>;
}

// Trait with associated constants
pub trait Geometry {
    const PI: f64 = 3.14159;
    
    fn area(&self) -> f64;
}

// ===== 9. TRAIT INHERITANCE =====

// Trait that extends another trait
pub trait AdvancedPrintable: Printable {
    fn print_with_border(&self) {
        println!("==================");
        self.print();
        println!("==================");
    }
}

// Implementing the extended trait
impl AdvancedPrintable for Article {}

// ===== 10. DEMONSTRATION FUNCTION =====

pub fn demonstrate_traits() {
    println!("🦀 RUST TRAIT SYSTEM DEEP STUDY 🦀\n");
    
    // ===== CREATING SAMPLE DATA =====
    let article = Article {
        title: "Understanding Rust Traits".to_string(),
        content: "Traits are a fundamental concept in Rust...".to_string(),
        author: "Rust Developer".to_string(),
    };
    
    let tweet = Tweet {
        username: "rustlang".to_string(),
        content: "Traits make Rust's type system incredibly powerful!".to_string(),
        reply_to: None,
    };
    
    let book = Book {
        title: "Programming Rust".to_string(),
        author: "Jim Blandy".to_string(),
        pages: 624,
    };
    
    // ===== DEMONSTRATING BASIC TRAIT USAGE =====
    println!("1️⃣ BASIC TRAIT IMPLEMENTATIONS:");
    article.print();
    tweet.print();
    book.print();
    
    println!("\n2️⃣ DEFAULT TRAIT METHODS:");
    article.print_twice();
    
    println!("\n3️⃣ ASSOCIATED FUNCTIONS:");
    println!("Article type: {}", Article::type_name());
    println!("Tweet type: {}", Tweet::type_name());
    println!("Book type: {}", Book::type_name());
    
    // ===== DEMONSTRATING GENERIC TRAIT BOUNDS =====
    println!("\n4️⃣ GENERIC FUNCTIONS WITH TRAIT BOUNDS:");
    // USING: <T: Trait> - Compile-time polymorphism
    // Each call gets its own optimized version (monomorphization)
    print_item(&article);   // Creates print_item::<Article>
    print_item(&tweet);     // Creates print_item::<Tweet>
    print_item(&book);      // Creates print_item::<Book>
    
    println!("\n5️⃣ MULTIPLE TRAIT BOUNDS:");
    // USING: <T> where T: Multiple + Bounds - Complex constraints
    // Only types implementing BOTH traits can be used
    print_and_summarize(&article);  // ✅ Article implements both
    print_and_summarize(&tweet);    // ✅ Tweet implements both
    // print_and_summarize(&book);  // ❌ Book doesn't implement Summarizable<String>
    
    println!("\n6️⃣ IMPL TRAIT SYNTAX:");
    // USING: &impl Trait - Syntactic sugar for generics
    // Same performance as generics, cleaner syntax
    print_with_impl_trait(&book);
    
    // ===== DEMONSTRATING SUMMARIZABLE TRAIT =====
    println!("\n7️⃣ SUMMARIZABLE TRAIT:");
    println!("Article summary: {}", article.summarize());
    println!("Tweet summary: {}", tweet.summarize());
    println!("Book pages: {}", book.summarize());
    
    // With context (requires Display trait bound)
    println!("Article with context: {}", article.summarize_with_context("Latest"));
    
    // ===== DEMONSTRATING DYNAMIC DISPATCH =====
    println!("\n8️⃣ DYNAMIC DISPATCH WITH TRAIT OBJECTS:");
    
    // Using trait objects directly
    // USING: &dyn Trait - Runtime polymorphism via vtable
    print_dynamic(&article);  // Calls Article::print via vtable
    print_dynamic(&tweet);    // Calls Tweet::print via vtable
    print_dynamic(&book);     // Calls Book::print via vtable
    
    // Vector of trait objects - heterogeneous collection
    // USING: Vec<&dyn Trait> - Store different types together
    // This is impossible with generics! Each element can be a different concrete type
    let printables: Vec<&dyn Printable> = vec![&article, &tweet, &book];
    println!("\n📦 Processing heterogeneous collection:");
    for (i, item) in printables.iter().enumerate() {
        println!("Item {}: ", i + 1);
        item.print();  // Runtime dispatch - don't know type until runtime
    }
    
    // Boxed trait objects
    // USING: Vec<Box<dyn Trait>> - Owned trait objects
    // Heap-allocated, can be moved, returned from functions
    println!("\n📦 Boxed trait objects:");
    let boxed_items: Vec<Box<dyn Printable>> = vec![
        create_printable_item(1),  // Returns Box<Article>
        create_printable_item(2),  // Returns Box<Tweet>
        create_printable_item(3),  // Returns Box<Book>
    ];
    
    for (i, item) in boxed_items.iter().enumerate() {
        println!("Boxed item {}: ", i + 1);
        item.print();  // Deref to &dyn Printable, then vtable call
    }
    
    // ===== DEMONSTRATING ADVANCED FEATURES =====
    println!("\n9️⃣ TRAIT INHERITANCE:");
    article.print_with_border();
    
    println!("\n🔟 KEY CONCEPTS SUMMARY:");
    println!("✅ Traits define shared behavior");
    println!("✅ Static dispatch: compile-time polymorphism (generics)");
    println!("✅ Dynamic dispatch: runtime polymorphism (trait objects)");
    println!("✅ Trait bounds constrain generic parameters");
    println!("✅ Default implementations provide fallback behavior");
    println!("✅ Associated types and constants add flexibility");
    println!("✅ Trait inheritance enables trait composition");
    
    println!("\n🎯 STATIC vs DYNAMIC DISPATCH:");
    println!("• Static (T: Trait): Fast, monomorphization, compile-time");
    println!("• Dynamic (&dyn Trait): Flexible, vtable lookup, runtime");
    
    println!("\n📊 TRAIT USAGE DECISION TREE:");
    println!("┌─ Need different types at runtime? ────── YES → &dyn Trait / Box<dyn Trait>");
    println!("│");
    println!("└─ NO → Compile-time known types");
    println!("    ├─ Simple function parameter? ───── YES → &impl Trait");
    println!("    ├─ Complex constraints? ─────────── YES → <T> where T: Multiple + Bounds");
    println!("    ├─ Performance critical? ────────── YES → <T: Trait>");
    println!("    └─ Default choice ─────────────────────── <T: Trait>");
    
    println!("\n🚀 PERFORMANCE COMPARISON:");
    println!("• <T: Trait>      : Zero-cost ✨ (inlined, optimized)");
    println!("• &impl Trait     : Zero-cost ✨ (same as above)");
    println!("• &dyn Trait      : Small cost 📊 (vtable lookup)");
    println!("• Box<dyn Trait>  : Higher cost 💰 (heap allocation + vtable)");
}
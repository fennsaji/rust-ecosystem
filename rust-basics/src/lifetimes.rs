// ===== LIFETIMES DEEP STUDY =====
// 
// WHAT ARE LIFETIMES?
// Lifetimes are annotations that tell the Rust compiler how long references
// should be valid. They prevent dangling references and ensure memory safety
// without a garbage collector. Every reference in Rust has a lifetime.

// ===== 1. BASIC LIFETIME CONCEPTS =====
// 
// UNDERSTANDING LIFETIME ANNOTATIONS:
// • 'a, 'b, 'static are lifetime parameters
// • They describe relationships between input and output lifetimes
// • They don't change how long values live, just describe existing relationships
// • The borrow checker uses them to ensure memory safety

// ===== 2. LONGEST STRING SLICE FUNCTION =====
// 
// This is the classic example for learning lifetimes
// Without lifetime annotations, this won't compile:
// fn longest(x: &str, y: &str) -> &str  // ❌ Missing lifetime specifier
// 
// The compiler needs to know which input lifetime the output relates to
pub fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    // LIFETIME EXPLANATION:
    // • 'a is a lifetime parameter (generic lifetime)
    // • Both x and y must live at least as long as 'a
    // • The return value will live as long as 'a
    // • This means the return value won't outlive either input
    // • The actual lifetime 'a will be the shorter of x and y's lifetimes
    
    if x.len() > y.len() {
        x  // Returns a reference with lifetime 'a
    } else {
        y  // Returns a reference with lifetime 'a
    }
}

// Alternative version showing different lifetime relationships
pub fn longest_with_different_lifetimes<'a, 'b>(x: &'a str, y: &'b str) -> &'a str 
where 
    'b: 'a,  // 'b must outlive 'a (lifetime bound)
{
    // This function always returns x, so output lifetime relates only to x
    // The 'b: 'a bound ensures y lives at least as long as the return value
    println!("Comparing: {} vs {}", x, y);
    x
}

// ===== 3. LIFETIME ELISION RULES =====
// 
// Rust has three rules for when you can omit lifetime annotations:
// 
// RULE 1: Each parameter that is a reference gets its own lifetime
// fn foo(x: &str, y: &str) becomes fn foo<'a, 'b>(x: &'a str, y: &'b str)
// 
// RULE 2: If there's exactly one input lifetime, it's assigned to all outputs
// fn foo(x: &str) -> &str becomes fn foo<'a>(x: &'a str) -> &'a str
// 
// RULE 3: If there's &self or &mut self, its lifetime is assigned to all outputs
// fn foo(&self, x: &str) -> &str becomes fn foo<'a>(&'a self, x: &str) -> &'a str

// Examples of lifetime elision:

// ELISION EXAMPLE 1: Single input parameter
// The compiler can infer the lifetime
pub fn first_word(s: &str) -> &str {
    // Elided: fn first_word<'a>(s: &'a str) -> &'a str
    let bytes = s.as_bytes();
    
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    
    &s[..]
}

// ELISION EXAMPLE 2: Multiple inputs, no elision possible
// This requires explicit lifetime annotations
pub fn longest_word_from_sentence<'a>(sentence: &'a str, word: &str) -> &'a str {
    // We need to specify that the output relates to 'sentence', not 'word'
    // Without annotation, compiler can't determine which input the output relates to
    
    let words: Vec<&str> = sentence.split_whitespace().collect();
    let mut longest = "";
    
    for w in words {
        if w.len() > longest.len() {
            longest = w;
        }
    }
    
    longest
}

// ELISION EXAMPLE 3: Method with &self
pub struct TextAnalyzer {
    text: String,
}

impl TextAnalyzer {
    pub fn new(text: String) -> Self {
        TextAnalyzer { text }
    }
    
    // Elided: fn get_text<'a>(&'a self) -> &'a str
    pub fn get_text(&self) -> &str {
        &self.text
    }
    
    // Elided: fn find_longest_word<'a>(&'a self, min_length: usize) -> Option<&'a str>
    pub fn find_longest_word(&self, min_length: usize) -> Option<&str> {
        self.text
            .split_whitespace()
            .filter(|word| word.len() >= min_length)
            .max_by_key(|word| word.len())
    }
}

// ===== 4. STRUCTS WITH LIFETIMES =====
// 
// When a struct holds references, it needs lifetime parameters
// This ensures the struct doesn't outlive the data it references

// BASIC STRUCT WITH LIFETIME
pub struct ImportantExcerpt<'a> {
    // The struct holds a reference to a string slice
    // The lifetime 'a ensures the struct doesn't outlive the referenced data
    part: &'a str,
}

impl<'a> ImportantExcerpt<'a> {
    // Constructor method
    pub fn new(part: &'a str) -> Self {
        ImportantExcerpt { part }
    }
    
    // Method returning reference with same lifetime as struct
    pub fn get_part(&self) -> &str {
        // Elided: fn get_part<'b>(&'b self) -> &'b str
        // But 'b is tied to 'a through the struct definition
        self.part
    }
    
    // Method that doesn't return references doesn't need lifetime annotations
    pub fn level(&self) -> i32 {
        3
    }
    
    // Method with multiple lifetimes
    pub fn announce_and_return_part<'b>(&self, announcement: &'b str) -> &str {
        // Returns reference tied to struct's lifetime, not parameter's lifetime
        println!("Attention please: {}", announcement);
        self.part
    }
}

// COMPLEX STRUCT WITH MULTIPLE LIFETIMES
pub struct BookExcerpt<'title, 'content> {
    // Different parts of the struct can have different lifetimes
    title: &'title str,
    content: &'content str,
    page_number: u32,
}

impl<'title, 'content> BookExcerpt<'title, 'content> {
    pub fn new(title: &'title str, content: &'content str, page_number: u32) -> Self {
        BookExcerpt {
            title,
            content,
            page_number,
        }
    }
    
    // Method returning reference with specific lifetime
    pub fn get_title(&self) -> &'title str {
        self.title
    }
    
    pub fn get_content(&self) -> &'content str {
        self.content
    }
    
    // Method combining both lifetimes
    pub fn format_excerpt(&self) -> String {
        format!("'{}' (page {}):\n{}", self.title, self.page_number, self.content)
    }
}

// ===== 5. STATIC LIFETIME =====
// 
// UNDERSTANDING 'static:
// • 'static means the reference can live for the entire program duration
// • String literals have 'static lifetime (stored in program binary)
// • Static variables also have 'static lifetime
// • Don't confuse 'static lifetime with static variables

// String literals have 'static lifetime
pub static GLOBAL_MESSAGE: &'static str = "This lives for the entire program";

// Function returning static reference
pub fn get_static_str() -> &'static str {
    "This string literal has 'static lifetime"
}

// Function that requires static lifetime
pub fn store_reference(r: &'static str) -> &'static str {
    // This function can only accept references that live for the entire program
    r
}

// ===== 6. LIFETIME BOUNDS =====
// 
// You can specify that one lifetime must outlive another

// Generic struct with lifetime bound
pub struct Holder<T>
where
    T: 'static,  // T must have 'static lifetime
{
    value: T,
}

impl<T> Holder<T>
where
    T: 'static,
{
    pub fn new(value: T) -> Self {
        Holder { value }
    }
    
    pub fn get_value(&self) -> &T {
        &self.value
    }
}

// Function with lifetime bounds
pub fn longer_than<'a, 'b>(x: &'a str, y: &'b str) -> &'a str
where
    'b: 'a,  // 'b must outlive 'a
{
    println!("y = {}", y);
    x  // We can use y because 'b: 'a guarantees it lives long enough
}

// ===== 7. COMMON LIFETIME ERRORS AND SOLUTIONS =====

// DANGLING REFERENCE ERROR (commented out - won't compile)
// fn dangling_reference() -> &str {
//     let s = String::from("hello");
//     &s  // ❌ Error: s is dropped at end of function
// }

// SOLUTION: Return owned value instead
pub fn no_dangling_reference() -> String {
    let s = String::from("hello");
    s  // ✅ Return owned value, not reference
}

// BORROWING ACROSS SCOPES ERROR (demonstration)
pub fn demonstrate_lifetime_error() {
    let string1 = String::from("long string is long");
    let result;
    
    {
        let string2 = String::from("xyz");
        result = longest(&string1, &string2);
        // string2 is dropped here, but result might try to reference it
        println!("The longest string is: {}", result);
    }
    
    // This would be an error if we tried to use result here:
    // println!("Result: {}", result); // ❌ string2 might be dropped
}

// SOLUTION: Ensure all references live long enough
pub fn demonstrate_lifetime_solution() {
    let string1 = String::from("long string is long");
    let string2 = String::from("xyz");
    
    // Both string1 and string2 live for the entire function
    let result = longest(&string1, &string2);
    println!("The longest string is: {}", result);
    
    // We can use result anywhere in this scope
    println!("Result is still valid: {}", result);
}

// ===== 8. DEMONSTRATION FUNCTION =====

pub fn demonstrate_lifetimes() {
    println!("🦀 RUST LIFETIMES DEEP STUDY 🦀\n");
    
    // ===== BASIC LIFETIME ANNOTATIONS =====
    println!("1️⃣ BASIC LIFETIME ANNOTATIONS:");
    let string1 = String::from("abcd");
    let string2 = "xyz";
    
    // Both strings live for the entire function, so this works
    let result = longest(&string1, &string2);
    println!("The longest string is: '{}'", result);
    
    // Different lifetimes example
    let long_string = String::from("long string is long");
    {
        let short_string = String::from("short");
        // This works because we use result within the scope where both strings live
        let result = longest(&long_string, &short_string);
        println!("Within scope, longest is: '{}'", result);
    }
    
    // ===== LIFETIME ELISION EXAMPLES =====
    println!("\n2️⃣ LIFETIME ELISION EXAMPLES:");
    let sentence = "Hello world from Rust programming";
    let first = first_word(sentence);
    println!("First word: '{}'", first);
    
    let longest_word = longest_word_from_sentence(sentence, "test");
    println!("Longest word in sentence: '{}'", longest_word);
    
    // ===== STRUCT WITH LIFETIMES =====
    println!("\n3️⃣ STRUCTS WITH LIFETIMES:");
    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.').next().expect("Could not find a '.'");
    
    // The excerpt references data from 'novel'
    let excerpt = ImportantExcerpt::new(first_sentence);
    println!("Important excerpt: '{}'", excerpt.get_part());
    println!("Excerpt level: {}", excerpt.level());
    
    // Method with announcement
    let announcement = "Listen up!";
    let part = excerpt.announce_and_return_part(announcement);
    println!("Returned part: '{}'", part);
    
    // ===== MULTIPLE LIFETIMES =====
    println!("\n4️⃣ MULTIPLE LIFETIMES:");
    let book_title = "The Rust Programming Language";
    let book_content = "Rust is a systems programming language...";
    
    let book_excerpt = BookExcerpt::new(book_title, book_content, 42);
    println!("Book title: '{}'", book_excerpt.get_title());
    println!("Book content: '{}'", book_excerpt.get_content());
    println!("Formatted excerpt:\n{}", book_excerpt.format_excerpt());
    
    // ===== STATIC LIFETIME =====
    println!("\n5️⃣ STATIC LIFETIME:");
    println!("Global message: '{}'", GLOBAL_MESSAGE);
    println!("Static string: '{}'", get_static_str());
    
    // Static holder
    let holder = Holder::new("This has static lifetime");
    println!("Holder value: '{}'", holder.get_value());
    
    // ===== TEXT ANALYZER =====
    println!("\n6️⃣ TEXT ANALYZER (ELISION IN METHODS):");
    let analyzer = TextAnalyzer::new("The quick brown fox jumps over the lazy dog".to_string());
    println!("Full text: '{}'", analyzer.get_text());
    
    if let Some(longest) = analyzer.find_longest_word(4) {
        println!("Longest word (min 4 chars): '{}'", longest);
    }
    
    // ===== LIFETIME SOLUTIONS =====
    println!("\n7️⃣ LIFETIME SOLUTIONS:");
    let owned_string = no_dangling_reference();
    println!("Owned string (no dangling): '{}'", owned_string);
    
    demonstrate_lifetime_solution();
    
    // ===== SUMMARY =====
    println!("\n🎯 LIFETIME CONCEPTS SUMMARY:");
    println!("✅ Lifetime annotations describe relationships, not durations");
    println!("✅ Elision rules let you omit annotations in many cases");
    println!("✅ Structs with references need lifetime parameters");
    println!("✅ 'static means 'lives for entire program duration'");
    println!("✅ Lifetime bounds specify outliving relationships");
    println!("✅ Dangling references are prevented at compile time");
    
    println!("\n📚 LIFETIME ELISION RULES:");
    println!("1. Each reference parameter gets its own lifetime");
    println!("2. One input lifetime → assigned to all outputs");
    println!("3. &self or &mut self → its lifetime assigned to all outputs");
    
    println!("\n🚫 COMMON LIFETIME ERRORS:");
    println!("• Returning references to local variables");
    println!("• Using references after their data is dropped");
    println!("• Struct outliving its referenced data");
    println!("• Mismatched lifetime relationships");
    
    println!("\n💡 LIFETIME BEST PRACTICES:");
    println!("• Prefer owned types over references when possible");
    println!("• Use lifetime elision when available");
    println!("• Make lifetime relationships explicit when needed");
    println!("• Understand the borrow checker's perspective");
}
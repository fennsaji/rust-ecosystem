// ===== SMART POINTERS DEEP STUDY =====
// 
// WHAT ARE SMART POINTERS?
// Smart pointers are data structures that act like pointers but have additional
// metadata and capabilities. They manage memory automatically and provide
// different ownership semantics than regular references.
// 
// RUST'S MAIN SMART POINTERS:
// • Box<T>: Heap allocation, single ownership
// • Rc<T>: Reference counting, shared ownership (single-threaded)
// • RefCell<T>: Interior mutability, runtime borrow checking
// • Arc<T>: Atomic reference counting, shared ownership (multi-threaded)
// • Weak<T>: Weak references to break cycles

use std::rc::{Rc, Weak};
use std::cell::RefCell;
use std::sync::{Arc, Mutex};
use std::thread;

// ===== 1. BOX<T> - HEAP ALLOCATION =====
// 
// UNDERSTANDING BOX<T>:
// • Stores data on the heap instead of the stack
// • Single ownership (like regular owned values)
// • Automatically deallocated when Box goes out of scope
// • Used for: large data, recursive types, trait objects
// 
// WHEN TO USE BOX<T>:
// • Data too large for stack
// • Recursive data structures
// • Trait objects when size unknown at compile time
// • Transfer ownership of heap data

// RECURSIVE LIST USING BOX<T>
// Without Box, this would be infinitely sized and won't compile
#[derive(Debug)]
pub enum List {
    // Box allows us to have a recursive type with known size
    // The Box itself has a fixed size (pointer), even though contents vary
    Cons(i32, Box<List>),  // Node with value and pointer to next
    Nil,                   // End of list
}

impl List {
    // Create a new empty list
    pub fn new() -> Self {
        List::Nil
    }
    
    // Add element to front of list
    pub fn cons(value: i32, list: List) -> Self {
        List::Cons(value, Box::new(list))
    }
    
    // Get length of list
    pub fn len(&self) -> usize {
        match self {
            List::Nil => 0,
            List::Cons(_, tail) => 1 + tail.len(),
        }
    }
    
    // Check if list is empty
    pub fn is_empty(&self) -> bool {
        matches!(self, List::Nil)
    }
    
    // Convert to Vec for easier printing
    pub fn to_vec(&self) -> Vec<i32> {
        match self {
            List::Nil => vec![],
            List::Cons(head, tail) => {
                let mut result = vec![*head];
                result.extend(tail.to_vec());
                result
            }
        }
    }
}

// BINARY TREE USING BOX<T>
#[derive(Debug)]
pub struct BinaryTree {
    value: i32,
    left: Option<Box<BinaryTree>>,
    right: Option<Box<BinaryTree>>,
}

impl BinaryTree {
    pub fn new(value: i32) -> Self {
        BinaryTree {
            value,
            left: None,
            right: None,
        }
    }
    
    pub fn insert(&mut self, value: i32) {
        if value < self.value {
            match &mut self.left {
                Some(left) => left.insert(value),
                None => self.left = Some(Box::new(BinaryTree::new(value))),
            }
        } else {
            match &mut self.right {
                Some(right) => right.insert(value),
                None => self.right = Some(Box::new(BinaryTree::new(value))),
            }
        }
    }
    
    pub fn contains(&self, value: i32) -> bool {
        if value == self.value {
            return true;
        }
        
        if value < self.value {
            self.left.as_ref().map_or(false, |left| left.contains(value))
        } else {
            self.right.as_ref().map_or(false, |right| right.contains(value))
        }
    }
}

// ===== 2. RC<T> - REFERENCE COUNTING =====
// 
// UNDERSTANDING RC<T>:
// • Multiple owners can share the same data
// • Reference counting tracks how many owners exist
// • Data is dropped when reference count reaches zero
// • Single-threaded only (not Send or Sync)
// • Immutable by default
// 
// WHEN TO USE RC<T>:
// • Multiple ownership needed
// • Single-threaded environment
// • Shared data that should live as long as any owner needs it
// • Graph-like data structures

// GRAPH NODE USING RC<T>
#[derive(Debug)]
pub struct Node {
    value: i32,
    // Vec of Rc allows multiple parents to point to same child
    children: Vec<Rc<Node>>,
}

impl Node {
    pub fn new(value: i32) -> Rc<Self> {
        Rc::new(Node {
            value,
            children: Vec::new(),
        })
    }
    
    // Note: We can't mutate through Rc directly
    // This is why we need RefCell for interior mutability
}

// ===== 3. REFCELL<T> - INTERIOR MUTABILITY =====
// 
// UNDERSTANDING REFCELL<T>:
// • Allows mutation of data even when there are immutable references
// • Enforces borrowing rules at runtime instead of compile time
// • Can panic at runtime if borrowing rules are violated
// • Single-threaded only
// 
// INTERIOR MUTABILITY PATTERN:
// • Rust's borrowing rules: either one mutable reference OR multiple immutable references
// • Sometimes we need to mutate data that appears immutable from the outside
// • RefCell moves borrow checking from compile time to runtime
// • Use .borrow() for immutable access, .borrow_mut() for mutable access
// 
// WHEN TO USE REFCELL<T>:
// • Need to mutate data through shared references
// • Mock objects in testing
// • Caching scenarios
// • When you know the code is safe but compiler can't prove it

// MUTABLE GRAPH NODE USING RC<REFCELL<T>>
#[derive(Debug)]
pub struct MutableNode {
    value: RefCell<i32>,  // Value can be mutated
    children: RefCell<Vec<Rc<MutableNode>>>,  // Children can be modified
    parent: RefCell<Option<Weak<MutableNode>>>,  // Weak reference to prevent cycles
}

impl MutableNode {
    pub fn new(value: i32) -> Rc<Self> {
        Rc::new(MutableNode {
            value: RefCell::new(value),
            children: RefCell::new(Vec::new()),
            parent: RefCell::new(None),
        })
    }
    
    pub fn add_child(parent: &Rc<MutableNode>, child: Rc<MutableNode>) {
        // Set parent reference in child (using Weak to avoid cycles)
        *child.parent.borrow_mut() = Some(Rc::downgrade(parent));
        
        // Add child to parent's children list
        parent.children.borrow_mut().push(child);
    }
    
    pub fn get_value(&self) -> i32 {
        *self.value.borrow()  // Immutable borrow
    }
    
    pub fn set_value(&self, new_value: i32) {
        *self.value.borrow_mut() = new_value;  // Mutable borrow
    }
    
    pub fn get_children_count(&self) -> usize {
        self.children.borrow().len()
    }
    
    pub fn get_children_values(&self) -> Vec<i32> {
        self.children
            .borrow()
            .iter()
            .map(|child| child.get_value())
            .collect()
    }
    
    pub fn has_parent(&self) -> bool {
        self.parent.borrow().is_some()
    }
}

// TREE STRUCTURE USING RC<REFCELL<T>>
#[derive(Debug)]
pub struct Tree {
    root: Option<Rc<TreeNode>>,
}

#[derive(Debug)]
pub struct TreeNode {
    value: RefCell<String>,
    children: RefCell<Vec<Rc<TreeNode>>>,
}

impl Tree {
    pub fn new() -> Self {
        Tree { root: None }
    }
    
    pub fn set_root(&mut self, value: String) {
        self.root = Some(Rc::new(TreeNode {
            value: RefCell::new(value),
            children: RefCell::new(Vec::new()),
        }));
    }
    
    pub fn get_root(&self) -> Option<Rc<TreeNode>> {
        self.root.clone()
    }
}

impl TreeNode {
    pub fn new(value: String) -> Rc<Self> {
        Rc::new(TreeNode {
            value: RefCell::new(value),
            children: RefCell::new(Vec::new()),
        })
    }
    
    pub fn add_child(&self, child: Rc<TreeNode>) {
        self.children.borrow_mut().push(child);
    }
    
    pub fn get_value(&self) -> String {
        self.value.borrow().clone()
    }
    
    pub fn set_value(&self, new_value: String) {
        *self.value.borrow_mut() = new_value;
    }
    
    pub fn get_children(&self) -> Vec<Rc<TreeNode>> {
        self.children.borrow().clone()
    }
    
    // Depth-first traversal
    pub fn traverse(&self, depth: usize) -> Vec<(String, usize)> {
        let mut result = vec![(self.get_value(), depth)];
        
        for child in self.get_children() {
            result.extend(child.traverse(depth + 1));
        }
        
        result
    }
}

// ===== 4. ARC<T> - ATOMIC REFERENCE COUNTING =====
// 
// UNDERSTANDING ARC<T>:
// • Thread-safe version of Rc<T>
// • Uses atomic operations for reference counting
// • Can be shared between threads (Send + Sync)
// • Slightly more expensive than Rc due to atomic operations
// 
// WHEN TO USE ARC<T>:
// • Multiple ownership needed across threads
// • Shared data in multi-threaded environment
// • Thread-safe shared state

// THREAD-SAFE COUNTER USING ARC<T>
pub struct SharedCounter {
    count: Arc<Mutex<i32>>,  // Arc for sharing, Mutex for thread-safe mutation
}

impl SharedCounter {
    pub fn new() -> Self {
        SharedCounter {
            count: Arc::new(Mutex::new(0)),
        }
    }
    
    pub fn get_count(&self) -> i32 {
        *self.count.lock().unwrap()
    }
    
    pub fn increment(&self) {
        let mut count = self.count.lock().unwrap();
        *count += 1;
    }
    
    pub fn clone_handle(&self) -> Self {
        SharedCounter {
            count: Arc::clone(&self.count),  // Clone the Arc, not the data
        }
    }
}

// SHARED DATA STRUCTURE FOR MULTI-THREADING
#[derive(Debug)]
pub struct SharedData {
    values: Arc<Mutex<Vec<i32>>>,
}

impl SharedData {
    pub fn new() -> Self {
        SharedData {
            values: Arc::new(Mutex::new(Vec::new())),
        }
    }
    
    pub fn add_value(&self, value: i32) {
        let mut values = self.values.lock().unwrap();
        values.push(value);
    }
    
    pub fn get_values(&self) -> Vec<i32> {
        let values = self.values.lock().unwrap();
        values.clone()
    }
    
    pub fn get_sum(&self) -> i32 {
        let values = self.values.lock().unwrap();
        values.iter().sum()
    }
    
    pub fn clone_handle(&self) -> Self {
        SharedData {
            values: Arc::clone(&self.values),
        }
    }
}

// ===== 5. WEAK<T> - WEAK REFERENCES =====
// 
// UNDERSTANDING WEAK<T>:
// • Weak references don't affect reference counting
// • Used to break reference cycles
// • Can be upgraded to strong references if data still exists
// • Automatically become invalid when strong references are dropped
// 
// WHEN TO USE WEAK<T>:
// • Parent-child relationships where child references parent
// • Breaking cycles in data structures
// • Observer patterns
// • Caching scenarios

// EXAMPLE: PARENT-CHILD WITH WEAK REFERENCES
pub struct Parent {
    children: RefCell<Vec<Rc<Child>>>,
}

pub struct Child {
    parent: RefCell<Option<Weak<Parent>>>,
    name: String,
}

impl Parent {
    pub fn new() -> Rc<Self> {
        Rc::new(Parent {
            children: RefCell::new(Vec::new()),
        })
    }
    
    pub fn add_child(parent: &Rc<Parent>, name: String) -> Rc<Child> {
        let child = Rc::new(Child {
            parent: RefCell::new(Some(Rc::downgrade(parent))),
            name,
        });
        
        parent.children.borrow_mut().push(child.clone());
        child
    }
    
    pub fn get_children_count(&self) -> usize {
        self.children.borrow().len()
    }
}

impl Child {
    pub fn get_name(&self) -> &str {
        &self.name
    }
    
    pub fn has_parent(&self) -> bool {
        self.parent.borrow().as_ref().map_or(false, |weak| weak.upgrade().is_some())
    }
    
    pub fn get_siblings_count(&self) -> Option<usize> {
        self.parent
            .borrow()
            .as_ref()?
            .upgrade()?
            .children
            .borrow()
            .len()
            .into()
    }
}

// ===== 6. SMART POINTER COMPARISON =====
// 
// DECISION MATRIX:
// 
// ┌─────────────┬─────────────┬─────────────┬─────────────┬─────────────┐
// │   Feature   │   Box<T>    │   Rc<T>     │ RefCell<T>  │   Arc<T>    │
// ├─────────────┼─────────────┼─────────────┼─────────────┼─────────────┤
// │ Ownership   │ Single      │ Multiple    │ Single      │ Multiple    │
// │ Mutability  │ Compile-time│ Immutable   │ Runtime     │ Immutable   │
// │ Thread Safe │ Yes         │ No          │ No          │ Yes         │
// │ Overhead    │ Minimal     │ Ref counting│ Borrow check│ Atomic ops  │
// │ Use Case    │ Heap alloc  │ Shared data │ Interior mut│ Thread share│
// └─────────────┴─────────────┴─────────────┴─────────────┴─────────────┘
// 
// COMMON COMBINATIONS:
// • Rc<RefCell<T>>: Shared mutable data (single-threaded)
// • Arc<Mutex<T>>: Shared mutable data (multi-threaded)
// • Box<dyn Trait>: Trait objects with single ownership
// • Rc<dyn Trait>: Trait objects with shared ownership

// ===== 7. DEMONSTRATION FUNCTION =====

pub fn demonstrate_smart_pointers() {
    println!("🦀 RUST SMART POINTERS DEEP STUDY 🦀\n");
    
    // ===== BOX<T> DEMONSTRATIONS =====
    println!("1️⃣ BOX<T> - HEAP ALLOCATION & RECURSIVE STRUCTURES:");
    
    // Simple Box usage
    let boxed_int = Box::new(42);
    println!("Boxed integer: {}", boxed_int);
    
    // Recursive list
    let list = List::cons(1, List::cons(2, List::cons(3, List::new())));
    println!("Recursive list: {:?}", list.to_vec());
    println!("List length: {}", list.len());
    
    // Binary tree
    let mut tree = BinaryTree::new(5);
    tree.insert(3);
    tree.insert(7);
    tree.insert(1);
    tree.insert(9);
    println!("Tree contains 3: {}", tree.contains(3));
    println!("Tree contains 6: {}", tree.contains(6));
    
    // ===== RC<T> DEMONSTRATIONS =====
    println!("\n2️⃣ RC<T> - REFERENCE COUNTING:");
    
    let node1 = Node::new(1);
    let node2 = Node::new(2);
    
    println!("Node1 reference count: {}", Rc::strong_count(&node1));
    
    // Clone creates new reference, doesn't copy data
    let node1_clone = Rc::clone(&node1);
    println!("Node1 reference count after clone: {}", Rc::strong_count(&node1));
    
    // Dropping clone decreases reference count
    drop(node1_clone);
    println!("Node1 reference count after drop: {}", Rc::strong_count(&node1));
    
    // ===== REFCELL<T> DEMONSTRATIONS =====
    println!("\n3️⃣ REFCELL<T> - INTERIOR MUTABILITY:");
    
    // Mutable node example
    let root = MutableNode::new(1);
    let child1 = MutableNode::new(2);
    let child2 = MutableNode::new(3);
    
    MutableNode::add_child(&root, child1);
    MutableNode::add_child(&root, child2);
    
    println!("Root value: {}", root.get_value());
    println!("Root children count: {}", root.get_children_count());
    println!("Children values: {:?}", root.get_children_values());
    
    // Modify value through RefCell
    root.set_value(10);
    println!("Root value after change: {}", root.get_value());
    
    // ===== RC<REFCELL<T>> TREE DEMONSTRATIONS =====
    println!("\n4️⃣ RC<REFCELL<T>> - SHARED MUTABLE TREE:");
    
    let mut tree = Tree::new();
    tree.set_root("root".to_string());
    
    if let Some(root) = tree.get_root() {
        let child1 = TreeNode::new("child1".to_string());
        let child2 = TreeNode::new("child2".to_string());
        let grandchild = TreeNode::new("grandchild".to_string());
        
        root.add_child(child1.clone());
        root.add_child(child2.clone());
        child1.add_child(grandchild);
        
        println!("Tree structure:");
        let traversal = root.traverse(0);
        for (value, depth) in traversal {
            println!("{}├─ {}", "  ".repeat(depth), value);
        }
        
        // Modify tree nodes
        child1.set_value("modified_child1".to_string());
        println!("After modification:");
        let traversal = root.traverse(0);
        for (value, depth) in traversal {
            println!("{}├─ {}", "  ".repeat(depth), value);
        }
    }
    
    // ===== ARC<T> DEMONSTRATIONS =====
    println!("\n5️⃣ ARC<T> - THREAD-SAFE SHARING:");
    
    let counter = SharedCounter::new();
    let mut handles = vec![];
    
    // Spawn threads that share the counter
    for i in 0..3 {
        let counter_clone = counter.clone_handle();
        let handle = thread::spawn(move || {
            for j in 0..5 {
                counter_clone.increment();
                println!("Thread {}: increment {}, count: {}", i, j + 1, counter_clone.get_count());
            }
        });
        handles.push(handle);
    }
    
    // Wait for all threads to complete
    for handle in handles {
        handle.join().unwrap();
    }
    
    println!("Final counter value: {}", counter.get_count());
    
    // Shared data example
    let shared_data = SharedData::new();
    let mut handles = vec![];
    
    for i in 0..3 {
        let data_clone = shared_data.clone_handle();
        let handle = thread::spawn(move || {
            for j in 0..3 {
                data_clone.add_value(i * 10 + j);
            }
        });
        handles.push(handle);
    }
    
    for handle in handles {
        handle.join().unwrap();
    }
    
    println!("Shared data values: {:?}", shared_data.get_values());
    println!("Sum: {}", shared_data.get_sum());
    
    // ===== WEAK<T> DEMONSTRATIONS =====
    println!("\n6️⃣ WEAK<T> - BREAKING CYCLES:");
    
    let parent = Parent::new();
    let child1 = Parent::add_child(&parent, "Alice".to_string());
    let child2 = Parent::add_child(&parent, "Bob".to_string());
    
    println!("Parent has {} children", parent.get_children_count());
    println!("Child1 name: {}", child1.get_name());
    println!("Child1 has parent: {}", child1.has_parent());
    println!("Child1 siblings count: {:?}", child1.get_siblings_count());
    
    // ===== SUMMARY =====
    println!("\n🎯 SMART POINTER CONCEPTS SUMMARY:");
    println!("✅ Box<T>: Single ownership, heap allocation");
    println!("✅ Rc<T>: Multiple ownership, reference counting");
    println!("✅ RefCell<T>: Interior mutability, runtime borrow checking");
    println!("✅ Arc<T>: Thread-safe multiple ownership");
    println!("✅ Weak<T>: Non-owning references, break cycles");
    
    println!("\n📊 USAGE PATTERNS:");
    println!("• Box<T>: Recursive types, large data, trait objects");
    println!("• Rc<RefCell<T>>: Shared mutable data (single-threaded)");
    println!("• Arc<Mutex<T>>: Shared mutable data (multi-threaded)");
    println!("• Weak<T>: Parent-child relationships, observer patterns");
    
    println!("\n🚫 COMMON PITFALLS:");
    println!("• RefCell runtime panics if borrowing rules violated");
    println!("• Reference cycles with Rc can cause memory leaks");
    println!("• Arc has performance overhead due to atomic operations");
    println!("• Weak references can become invalid");
    
    println!("\n💡 BEST PRACTICES:");
    println!("• Use owned types when possible");
    println!("• Prefer Box<T> for single ownership");
    println!("• Use Rc<T> for shared immutable data");
    println!("• Combine with RefCell for shared mutable data");
    println!("• Use Arc<T> only when threads involved");
    println!("• Use Weak<T> to break cycles");
}
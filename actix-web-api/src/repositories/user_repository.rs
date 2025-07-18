//! # Repository Layer - Data Access Abstraction
//! 
//! This module defines the **Repository Pattern** for data access in our Clean Architecture.
//! The repository layer is responsible for:
//! 
//! 1. **Data Access Abstraction**: Hiding database implementation details
//! 2. **Data Persistence**: Converting between domain models and storage formats
//! 3. **Query Operations**: Providing data access methods for business logic
//! 4. **Data Integrity**: Ensuring data consistency and constraints
//! 
//! ## Clean Architecture Position:
//! ```
//! HTTP Request → Routes → Handlers → Services → **[REPOSITORIES]** → Database
//! ```
//! 
//! ## Key Design Patterns:
//! - **Repository Pattern**: Abstracts data access behind a clean interface
//! - **Dependency Inversion**: Services depend on repository abstractions
//! - **Async Operations**: All data access is asynchronous
//! - **Thread Safety**: Uses Arc<RwLock> for concurrent access

use crate::errors::{AppError, AppResult};
use crate::models::{CreateUserDto, UpdateUserDto, User};
use async_trait::async_trait;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use uuid::Uuid;

/// User Repository Trait
/// 
/// This trait defines the **contract** for user data access operations.
/// It represents the **Repository Layer** in Clean Architecture.
/// 
/// ## Repository Pattern Benefits:
/// - **Testability**: Easy to create mock implementations for testing
/// - **Flexibility**: Can swap storage implementations (memory, database, etc.)
/// - **Separation of Concerns**: Business logic doesn't know about data storage
/// - **Query Abstraction**: Provides domain-specific query methods
/// 
/// ## Async Trait Pattern:
/// - `#[async_trait]`: Required for async methods in traits
/// - `Send + Sync`: Ensures thread safety for concurrent access
/// - All methods return `AppResult<T>` for consistent error handling
#[async_trait]
pub trait UserRepository: Send + Sync {
    /// Creates a new user in the data store
    /// 
    /// **Business Rules Enforced:**
    /// - Email uniqueness validation
    /// - Automatic ID generation
    /// - Timestamp management
    async fn create(&self, create_dto: CreateUserDto) -> AppResult<User>;
    
    /// Finds a user by their unique identifier
    /// 
    /// **Returns:**
    /// - `Some(User)` if found
    /// - `None` if not found
    /// - `Err` if database error occurs
    async fn find_by_id(&self, id: Uuid) -> AppResult<Option<User>>;
    
    /// Finds a user by their email address
    /// 
    /// **Use Cases:**
    /// - Login authentication
    /// - Email uniqueness validation
    /// - User lookup by email
    async fn find_by_email(&self, email: &str) -> AppResult<Option<User>>;
    
    /// Retrieves all users from the data store
    /// 
    /// **Note:** In production, this should support pagination
    /// to avoid loading large datasets into memory
    async fn find_all(&self) -> AppResult<Vec<User>>;
    
    /// Updates an existing user's information
    /// 
    /// **Business Rules Enforced:**
    /// - User existence validation
    /// - Email uniqueness validation (if email is being updated)
    /// - Automatic timestamp updates
    async fn update(&self, id: Uuid, update_dto: UpdateUserDto) -> AppResult<User>;
    
    /// Deletes a user from the data store
    /// 
    /// **Business Rules Enforced:**
    /// - User existence validation
    /// - Cascade deletion (if applicable)
    async fn delete(&self, id: Uuid) -> AppResult<()>;
    
    /// Checks if a user exists with the given email
    /// 
    /// **Optimization:** This method is more efficient than `find_by_email`
    /// when you only need to check existence
    async fn exists_by_email(&self, email: &str) -> AppResult<bool>;
}

/// In-Memory Repository Implementation
/// 
/// This implementation uses a `HashMap` for data storage, wrapped in
/// `Arc<RwLock>` for thread-safe concurrent access.
/// 
/// ## Thread Safety Pattern:
/// - `Arc`: Allows shared ownership across multiple threads
/// - `RwLock`: Allows multiple readers OR one writer (not both)
/// - `HashMap`: Fast key-value storage for user data
/// 
/// ## When to Use:
/// - **Development**: Quick setup without database dependencies
/// - **Testing**: Fast, isolated test runs
/// - **Prototyping**: Rapid development without database setup
/// 
/// ## Limitations:
/// - **No Persistence**: Data is lost when application stops
/// - **No Transactions**: No ACID properties
/// - **Memory Usage**: All data stored in memory
pub struct InMemoryUserRepository {
    // Thread-safe storage for user data
    // Arc<RwLock<HashMap>> allows multiple readers or one writer
    users: Arc<RwLock<HashMap<Uuid, User>>>,
}

impl InMemoryUserRepository {
    /// Creates a new in-memory repository
    /// 
    /// **Thread Safety Setup:**
    /// - `HashMap::new()`: Creates empty storage
    /// - `RwLock::new()`: Wraps storage for concurrent access
    /// - `Arc::new()`: Enables sharing across threads
    pub fn new() -> Self {
        Self {
            users: Arc::new(RwLock::new(HashMap::new())),
        }
    }
}

/// Default implementation for convenience
impl Default for InMemoryUserRepository {
    fn default() -> Self {
        Self::new()
    }
}

/// Repository Implementation for In-Memory Storage
/// 
/// This implementation demonstrates all repository operations
/// using in-memory storage with proper async/await patterns.
#[async_trait]
impl UserRepository for InMemoryUserRepository {
    /// Create User Implementation
    /// 
    /// **Steps:**
    /// 1. Acquire write lock (exclusive access)
    /// 2. Check email uniqueness constraint
    /// 3. Create new user with generated ID
    /// 4. Store user in HashMap
    /// 5. Return created user
    async fn create(&self, create_dto: CreateUserDto) -> AppResult<User> {
        // Acquire write lock for exclusive access
        // This blocks other writers but allows us to modify the HashMap
        let mut users = self.users.write().await;
        
        // Business Rule: Email must be unique
        // Check if any existing user has the same email
        if users.values().any(|u| u.email == create_dto.email) {
            return Err(AppError::UserAlreadyExists {
                email: create_dto.email,
            });
        }
        
        // Create new user with generated ID and timestamps
        let user = User::new(create_dto.email, create_dto.name);
        
        // Store user in HashMap using ID as key
        users.insert(user.id, user.clone());
        
        // Return the created user
        Ok(user)
    }
    
    /// Find User by ID Implementation
    /// 
    /// **Steps:**
    /// 1. Acquire read lock (shared access)
    /// 2. Look up user by ID in HashMap
    /// 3. Return cloned user if found, None if not found
    async fn find_by_id(&self, id: Uuid) -> AppResult<Option<User>> {
        // Acquire read lock for shared access
        // Multiple threads can read simultaneously
        let users = self.users.read().await;
        
        // Look up user by ID and clone if found
        // .cloned() is needed because we can't return a reference
        // that outlives the lock guard
        Ok(users.get(&id).cloned())
    }
    
    /// Find User by Email Implementation
    /// 
    /// **Steps:**
    /// 1. Acquire read lock (shared access)
    /// 2. Search through all users for matching email
    /// 3. Return cloned user if found, None if not found
    async fn find_by_email(&self, email: &str) -> AppResult<Option<User>> {
        // Acquire read lock for shared access
        let users = self.users.read().await;
        
        // Search through all users for matching email
        // This is O(n) operation - in a real database, this would be indexed
        Ok(users.values().find(|u| u.email == email).cloned())
    }
    
    /// Find All Users Implementation
    /// 
    /// **Steps:**
    /// 1. Acquire read lock (shared access)
    /// 2. Clone all users from HashMap
    /// 3. Return as vector
    /// 
    /// **Note:** In production, this should support pagination
    async fn find_all(&self) -> AppResult<Vec<User>> {
        // Acquire read lock for shared access
        let users = self.users.read().await;
        
        // Clone all users and collect into vector
        // This creates a snapshot of all users at this moment
        Ok(users.values().cloned().collect())
    }
    
    /// Update User Implementation
    /// 
    /// **Steps:**
    /// 1. Acquire write lock (exclusive access)
    /// 2. Check email uniqueness if email is being updated
    /// 3. Find user by ID
    /// 4. Update user data
    /// 5. Return updated user
    async fn update(&self, id: Uuid, update_dto: UpdateUserDto) -> AppResult<User> {
        // Acquire write lock for exclusive access
        let mut users = self.users.write().await;
        
        // Business Rule: Email must be unique (if being updated)
        if let Some(ref new_email) = update_dto.email {
            // Check if any OTHER user has this email
            if users.values().any(|u| u.id != id && u.email == *new_email) {
                return Err(AppError::UserAlreadyExists {
                    email: new_email.clone(),
                });
            }
        }
        
        // Find and update the user
        match users.get_mut(&id) {
            Some(user) => {
                // Update user data using domain model method
                user.update(update_dto);
                // Return cloned updated user
                Ok(user.clone())
            }
            None => {
                // User not found - return domain error
                Err(AppError::UserNotFound { id })
            }
        }
    }
    
    /// Delete User Implementation
    /// 
    /// **Steps:**
    /// 1. Acquire write lock (exclusive access)
    /// 2. Remove user from HashMap
    /// 3. Return success or error based on whether user existed
    async fn delete(&self, id: Uuid) -> AppResult<()> {
        // Acquire write lock for exclusive access
        let mut users = self.users.write().await;
        
        // Remove user from HashMap
        match users.remove(&id) {
            Some(_) => {
                // User was found and removed
                Ok(())
            }
            None => {
                // User not found - return domain error
                Err(AppError::UserNotFound { id })
            }
        }
    }
    
    /// Check User Existence by Email Implementation
    /// 
    /// **Steps:**
    /// 1. Acquire read lock (shared access)
    /// 2. Search for user with matching email
    /// 3. Return boolean result
    /// 
    /// **Optimization:** This is more efficient than `find_by_email`
    /// when you only need to check existence
    async fn exists_by_email(&self, email: &str) -> AppResult<bool> {
        // Acquire read lock for shared access
        let users = self.users.read().await;
        
        // Check if any user has this email
        // Returns true/false instead of Option<User>
        Ok(users.values().any(|u| u.email == email))
    }
}
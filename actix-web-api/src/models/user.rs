//! # Domain Models and Data Transfer Objects
//! 
//! This module contains the **domain models** and **DTOs** for our Clean Architecture.
//! It's responsible for:
//! 
//! 1. **Domain Models**: Core business entities that represent real-world concepts
//! 2. **Data Transfer Objects**: Structures for API input/output and data transformation
//! 3. **Data Validation**: Structural validation through type system
//! 4. **Serialization**: Converting between internal models and JSON/other formats
//! 
//! ## Clean Architecture Position:
//! ```
//! Domain Models: Central to all layers
//! DTOs: Interface between layers (API ↔ Service ↔ Repository)
//! ```
//! 
//! ## Key Design Patterns:
//! - **Domain-Driven Design**: User represents a core business concept
//! - **DTO Pattern**: Separate models for input, output, and internal use
//! - **Builder Pattern**: Domain models can be constructed with factory methods
//! - **Immutability**: Most fields are immutable except through specific methods

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// User Domain Model
/// 
/// This is the **core domain entity** representing a User in our system.
/// It contains the complete business representation of a user with all
/// fields that are important to our domain.
/// 
/// ## Domain-Driven Design:
/// - Represents a real-world business entity
/// - Contains all business-relevant data
/// - Provides methods for domain operations
/// - Maintains data integrity and business rules
/// 
/// ## Serde Annotations:
/// - `#[derive(Serialize, Deserialize)]`: Enables JSON conversion
/// - This allows the model to be serialized to/from JSON
/// - Used for database storage and API responses
/// 
/// ## Field Meanings:
/// - `id`: Unique identifier (UUID v4 for global uniqueness)
/// - `email`: User's email address (unique business identifier)
/// - `name`: User's display name
/// - `created_at`: When the user was first created (audit trail)
/// - `updated_at`: When the user was last modified (audit trail)
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct User {
    pub id: Uuid,
    pub email: String,
    pub name: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// Create User Data Transfer Object
/// 
/// This DTO represents the **input data** required to create a new user.
/// It only contains the fields that can be provided by the client,
/// excluding generated fields like ID and timestamps.
/// 
/// ## DTO Pattern Benefits:
/// - **Validation**: Only contains valid input fields
/// - **Security**: Prevents clients from setting system-generated fields
/// - **API Clarity**: Clear contract for what data is required
/// - **Evolution**: Can change independently from domain model
/// 
/// ## Serde Annotations:
/// - `Deserialize`: Converts JSON input to this struct
/// - `Serialize`: Allows converting back to JSON (useful for testing)
#[derive(Debug, Deserialize, Serialize)]
pub struct CreateUserDto {
    pub email: String,
    pub name: String,
}

/// Update User Data Transfer Object
/// 
/// This DTO represents the **input data** for updating an existing user.
/// It uses `Option<T>` for fields to support **partial updates** - only
/// provided fields will be updated.
/// 
/// ## Partial Update Pattern:
/// - `Option<String>`: Field can be omitted (None) or updated (Some(value))
/// - `None` means "don't change this field"
/// - `Some(value)` means "update to this value"
/// - This enables PATCH-style updates in REST APIs
/// 
/// ## Business Rules:
/// - At least one field must be provided (enforced in service layer)
/// - Email must be unique if provided (enforced in repository layer)
/// - Name cannot be empty if provided (enforced in service layer)
#[derive(Debug, Deserialize, Serialize)]
pub struct UpdateUserDto {
    pub email: Option<String>,
    pub name: Option<String>,
}

/// User Response Data Transfer Object
/// 
/// This DTO represents the **output data** returned by the API.
/// It contains all user information that should be exposed to clients.
/// 
/// ## Response DTO Pattern:
/// - **Separation**: Separate from domain model for API evolution
/// - **Control**: Explicit control over what data is exposed
/// - **Consistency**: Consistent response format across endpoints
/// - **Documentation**: Clear API contract for responses
/// 
/// ## Why separate from domain model?
/// - Domain model might contain sensitive fields
/// - API responses might need different formatting
/// - Allows independent evolution of internal and external models
#[derive(Debug, Serialize)]
pub struct UserResponseDto {
    pub id: Uuid,
    pub email: String,
    pub name: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// Users List Response Data Transfer Object
/// 
/// This DTO represents the **output data** for list operations.
/// It includes both the user data and metadata about the collection.
/// 
/// ## Collection Response Pattern:
/// - `users`: The actual data collection
/// - `total`: Metadata about the collection size
/// - Could be extended with pagination info (offset, limit, etc.)
/// - Consistent structure for all list operations
/// 
/// ## Future Extensions:
/// - Add pagination fields (page, per_page, total_pages)
/// - Add filtering metadata (applied_filters)
/// - Add sorting metadata (sort_by, sort_order)
#[derive(Debug, Serialize)]
pub struct UsersListResponseDto {
    pub users: Vec<UserResponseDto>,
    pub total: usize,
}

/// Conversion from Domain Model to Response DTO
/// 
/// This implementation demonstrates the **mapping pattern** between
/// internal domain models and external API responses.
/// 
/// ## From Trait Pattern:
/// - `From<User>` enables `UserResponseDto::from(user)`
/// - Provides explicit conversion between model types
/// - Ensures consistent mapping logic
/// - Makes conversions easy and discoverable
/// 
/// ## Why explicit conversion?
/// - Control over what fields are exposed
/// - Opportunity to transform data if needed
/// - Clear separation between internal and external models
/// - Easy to modify conversion logic
impl From<User> for UserResponseDto {
    fn from(user: User) -> Self {
        Self {
            id: user.id,
            email: user.email,
            name: user.name,
            created_at: user.created_at,
            updated_at: user.updated_at,
        }
    }
}

/// User Domain Model Implementation
/// 
/// This implementation contains the **business logic** for User operations.
/// It provides factory methods and business operations.
impl User {
    /// Creates a new user with generated ID and timestamps
    /// 
    /// This is a **factory method** that creates a new User instance
    /// with proper defaults and generated values.
    /// 
    /// ## Factory Pattern Benefits:
    /// - **Encapsulation**: Hides complex object creation logic
    /// - **Consistency**: Ensures all users are created with proper defaults
    /// - **Validation**: Could add validation logic here
    /// - **Immutability**: Creates a valid object in one step
    /// 
    /// ## Generated Fields:
    /// - `id`: UUID v4 (globally unique, no database dependency)
    /// - `created_at`: Current UTC timestamp
    /// - `updated_at`: Same as created_at initially
    /// 
    /// ## UUID Benefits:
    /// - Globally unique across all systems
    /// - No database round-trip needed for generation
    /// - Safe to generate in distributed systems
    /// - Hard to guess (security benefit)
    pub fn new(email: String, name: String) -> Self {
        let now = Utc::now();
        Self {
            // Generate a new UUID v4 (random)
            id: Uuid::new_v4(),
            email,
            name,
            // Set both timestamps to current time
            created_at: now,
            updated_at: now,
        }
    }

    /// Updates user fields and timestamp
    /// 
    /// This method implements the **business logic** for updating a user.
    /// It demonstrates the **partial update pattern** and **audit trail** management.
    /// 
    /// ## Business Rules Implemented:
    /// - Only update provided fields (partial update)
    /// - Always update the `updated_at` timestamp
    /// - Maintain original `created_at` timestamp
    /// 
    /// ## Partial Update Pattern:
    /// - `Option<T>` fields are only updated if `Some(value)` is provided
    /// - `None` values are ignored (field remains unchanged)
    /// - This enables PATCH-style updates in REST APIs
    /// 
    /// ## Audit Trail:
    /// - `updated_at` is always updated to current time
    /// - `created_at` is never changed (immutable audit record)
    /// - This provides a complete audit trail of changes
    pub fn update(&mut self, update_dto: UpdateUserDto) {
        // Update email if provided
        if let Some(email) = update_dto.email {
            self.email = email;
        }
        
        // Update name if provided
        if let Some(name) = update_dto.name {
            self.name = name;
        }
        
        // Always update the timestamp when any field is updated
        self.updated_at = Utc::now();
    }
}
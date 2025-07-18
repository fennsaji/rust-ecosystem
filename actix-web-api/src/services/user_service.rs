//! # Business Logic Service Layer
//! 
//! This module contains the **business logic layer** of our Clean Architecture.
//! The service layer is responsible for:
//! 
//! 1. **Business Rules**: Implementing domain-specific logic and constraints
//! 2. **Input Validation**: Ensuring data meets business requirements
//! 3. **Orchestration**: Coordinating calls to repositories and other services
//! 4. **Domain Logic**: Converting between domain models and DTOs
//! 
//! ## Clean Architecture Position:
//! ```
//! HTTP Request → Routes → Handlers → **[SERVICES]** → Repositories → Database
//! ```
//! 
//! ## Key Design Patterns:
//! - **Dependency Injection**: Services depend on repository abstractions
//! - **Trait-based Design**: Service interface is defined by a trait
//! - **Validation**: Business rules are enforced here, not in handlers
//! - **Error Handling**: Domain-specific errors are returned

use crate::errors::{invalid_input, validation_error, AppError, AppResult};
use crate::models::{CreateUserDto, UpdateUserDto, UserResponseDto, UsersListResponseDto};
use crate::repositories::UserRepository;
use async_trait::async_trait;
use std::sync::Arc;
use uuid::Uuid;

/// User Service Trait
/// 
/// This trait defines the **contract** for user business operations.
/// It represents the **Service Layer** in Clean Architecture.
/// 
/// ## Why use a trait?
/// - **Testability**: Easy to create mock implementations for testing
/// - **Flexibility**: Can swap implementations (e.g., different business rules)
/// - **Dependency Inversion**: Higher layers depend on abstractions, not concrete types
/// 
/// ## Async Trait Pattern:
/// - `#[async_trait]`: Enables async functions in traits (required for async methods)
/// - `Send + Sync`: Ensures the trait can be used across threads (required for Actix-Web)
#[async_trait]
pub trait UserService: Send + Sync {
    /// Creates a new user with business validation
    async fn create_user(&self, create_dto: CreateUserDto) -> AppResult<UserResponseDto>;
    
    /// Retrieves a user by their unique identifier
    async fn get_user_by_id(&self, id: Uuid) -> AppResult<UserResponseDto>;
    
    /// Retrieves all users with pagination information
    async fn get_all_users(&self) -> AppResult<UsersListResponseDto>;
    
    /// Updates an existing user with business validation
    async fn update_user(&self, id: Uuid, update_dto: UpdateUserDto) -> AppResult<UserResponseDto>;
    
    /// Deletes a user from the system
    async fn delete_user(&self, id: Uuid) -> AppResult<()>;
}

/// User Service Implementation
/// 
/// This struct implements the business logic for user operations.
/// It demonstrates the **Repository Pattern** by depending on a repository abstraction.
/// 
/// ## Dependency Injection Pattern:
/// The service receives a repository implementation via its constructor,
/// following the **Dependency Inversion Principle**.
pub struct UserServiceImpl {
    // Repository dependency - note we depend on the trait, not a concrete type
    repository: Arc<dyn UserRepository>,
}

impl UserServiceImpl {
    /// Creates a new UserService with the provided repository
    /// 
    /// ## Constructor Injection Pattern:
    /// This is a common dependency injection pattern where dependencies
    /// are provided through the constructor.
    pub fn new(repository: Arc<dyn UserRepository>) -> Self {
        Self { repository }
    }
    
    /// Email Validation Business Rule
    /// 
    /// This function encapsulates the business rules for email validation.
    /// It's a private function that enforces domain-specific constraints.
    /// 
    /// ## Business Rules Implemented:
    /// - Email cannot be empty
    /// - Email must contain @ symbol (basic format check)
    /// - Email cannot exceed 254 characters (RFC 5321 limit)
    fn validate_email(email: &str) -> AppResult<()> {
        // Business Rule: Email is required
        if email.is_empty() {
            return Err(validation_error("email", "Email cannot be empty"));
        }
        
        // Business Rule: Email must have basic format
        if !email.contains('@') {
            return Err(validation_error("email", "Invalid email format"));
        }
        
        // Business Rule: Email length limit (RFC 5321)
        if email.len() > 254 {
            return Err(validation_error("email", "Email too long"));
        }
        
        Ok(())
    }
    
    /// Name Validation Business Rule
    /// 
    /// This function encapsulates the business rules for name validation.
    /// 
    /// ## Business Rules Implemented:
    /// - Name cannot be empty
    /// - Name cannot exceed 100 characters
    /// - Name cannot be only whitespace
    fn validate_name(name: &str) -> AppResult<()> {
        // Business Rule: Name is required
        if name.is_empty() {
            return Err(validation_error("name", "Name cannot be empty"));
        }
        
        // Business Rule: Name length limit
        if name.len() > 100 {
            return Err(validation_error("name", "Name too long"));
        }
        
        // Business Rule: Name must have actual content
        if name.trim().is_empty() {
            return Err(validation_error("name", "Name cannot be only whitespace"));
        }
        
        Ok(())
    }
    
    /// Create User DTO Validation
    /// 
    /// This function validates all fields required for creating a user.
    /// It demonstrates **composite validation** - validating multiple fields together.
    fn validate_create_user_dto(dto: &CreateUserDto) -> AppResult<()> {
        // Validate email using business rules
        Self::validate_email(&dto.email)?;
        // Validate name using business rules
        Self::validate_name(&dto.name)?;
        Ok(())
    }
    
    /// Update User DTO Validation
    /// 
    /// This function validates update operations with different rules than create.
    /// It demonstrates **conditional validation** based on which fields are provided.
    fn validate_update_user_dto(dto: &UpdateUserDto) -> AppResult<()> {
        // Business Rule: At least one field must be provided for update
        if dto.email.is_none() && dto.name.is_none() {
            return Err(invalid_input("At least one field must be provided for update"));
        }
        
        // Validate email if provided (optional field in update)
        if let Some(ref email) = dto.email {
            Self::validate_email(email)?;
        }
        
        // Validate name if provided (optional field in update)
        if let Some(ref name) = dto.name {
            Self::validate_name(name)?;
        }
        
        Ok(())
    }
}

/// Service Implementation
/// 
/// This implementation contains the actual business logic for each operation.
/// Notice how it follows a consistent pattern:
/// 1. Validate input according to business rules
/// 2. Delegate to repository for data operations
/// 3. Transform results to appropriate DTOs
#[async_trait]
impl UserService for UserServiceImpl {
    /// Create User Business Logic
    /// 
    /// This method implements the complete business process for creating a user:
    /// 1. Validate input according to business rules
    /// 2. Delegate to repository for persistence
    /// 3. Transform domain model to response DTO
    async fn create_user(&self, create_dto: CreateUserDto) -> AppResult<UserResponseDto> {
        // Step 1: Validate input according to business rules
        // This happens in the service layer, not the handler layer
        Self::validate_create_user_dto(&create_dto)?;
        
        // Step 2: Delegate to repository for data persistence
        // The repository handles database-specific operations
        let user = self.repository.create(create_dto).await?;
        
        // Step 3: Transform domain model to response DTO
        // This separates internal models from API responses
        Ok(UserResponseDto::from(user))
    }
    
    /// Get User by ID Business Logic
    /// 
    /// This method demonstrates **error handling** in the service layer.
    /// It converts repository results to appropriate domain errors.
    async fn get_user_by_id(&self, id: Uuid) -> AppResult<UserResponseDto> {
        // Delegate to repository to find the user
        match self.repository.find_by_id(id).await? {
            Some(user) => {
                // User found: convert to response DTO
                Ok(UserResponseDto::from(user))
            }
            None => {
                // User not found: return domain-specific error
                // This is a business logic concern, not a database concern
                Err(AppError::UserNotFound { id })
            }
        }
    }
    
    /// Get All Users Business Logic
    /// 
    /// This method demonstrates **data transformation** in the service layer.
    /// It converts a list of domain models to a paginated response DTO.
    async fn get_all_users(&self) -> AppResult<UsersListResponseDto> {
        // Delegate to repository to get all users
        let users = self.repository.find_all().await?;
        
        // Transform domain models to response DTOs
        let user_dtos: Vec<UserResponseDto> = users
            .into_iter()
            .map(UserResponseDto::from)
            .collect();
        
        // Calculate metadata (could add pagination logic here)
        let total = user_dtos.len();
        
        // Return structured response with data and metadata
        Ok(UsersListResponseDto {
            users: user_dtos,
            total,
        })
    }
    
    /// Update User Business Logic
    /// 
    /// This method demonstrates **validation** and **delegation** patterns.
    /// It validates partial updates and delegates to the repository.
    async fn update_user(&self, id: Uuid, update_dto: UpdateUserDto) -> AppResult<UserResponseDto> {
        // Step 1: Validate input for update operations
        Self::validate_update_user_dto(&update_dto)?;
        
        // Step 2: Delegate to repository for data update
        let user = self.repository.update(id, update_dto).await?;
        
        // Step 3: Transform updated domain model to response DTO
        Ok(UserResponseDto::from(user))
    }
    
    /// Delete User Business Logic
    /// 
    /// This method demonstrates **simple delegation** to the repository.
    /// In a more complex system, this might check business rules before deletion.
    async fn delete_user(&self, id: Uuid) -> AppResult<()> {
        // Delegate to repository for deletion
        // In a real system, you might check:
        // - User permissions
        // - Related data that needs cleanup
        // - Business rules about deletion
        self.repository.delete(id).await
    }
}
//! # Error Handling and HTTP Response Mapping
//! 
//! This module defines the **error handling strategy** for our Clean Architecture application.
//! It's responsible for:
//! 
//! 1. **Domain Errors**: Defining application-specific error types
//! 2. **HTTP Mapping**: Converting domain errors to HTTP responses
//! 3. **Error Consistency**: Providing consistent error responses across the API
//! 4. **Error Context**: Maintaining detailed error information for debugging
//! 
//! ## Clean Architecture Position:
//! ```
//! Errors flow through all layers: Repository → Service → Handler → HTTP Response
//! ```
//! 
//! ## Key Error Handling Patterns:
//! - **Custom Error Types**: Using `thiserror` for ergonomic error definitions
//! - **HTTP Integration**: `ResponseError` trait for Actix-Web integration
//! - **Structured Errors**: Consistent JSON error response format
//! - **Error Propagation**: `?` operator for clean error propagation

use actix_web::{HttpResponse, ResponseError};
use serde_json::json;
use thiserror::Error;
use uuid::Uuid;

/// Application Error Types
/// 
/// This enum defines all possible errors that can occur in our application.
/// It uses the `thiserror` crate for ergonomic error handling.
/// 
/// ## Thiserror Benefits:
/// - `#[derive(Error)]`: Automatically implements `std::error::Error`
/// - `#[error("...")]`: Defines display messages with interpolation
/// - Structured error data with named fields
/// - Automatic `From` implementations for error conversion
/// 
/// ## Error Categories:
/// - **Business Logic Errors**: Domain-specific errors (UserNotFound, etc.)
/// - **Validation Errors**: Input validation failures
/// - **Infrastructure Errors**: Database, network, etc.
/// - **System Errors**: Unexpected internal errors
/// 
/// ## Each Error Contains:
/// - Descriptive error message
/// - Relevant context data (IDs, field names, etc.)
/// - Structured data for programmatic handling
#[derive(Error, Debug)]
pub enum AppError {
    /// User Not Found Error
    /// 
    /// **When**: Trying to access a user that doesn't exist
    /// **HTTP Status**: 404 Not Found
    /// **Context**: User ID that was requested
    #[error("User not found: {id}")]
    UserNotFound { id: Uuid },
    
    /// User Already Exists Error
    /// 
    /// **When**: Trying to create a user with an email that already exists
    /// **HTTP Status**: 409 Conflict
    /// **Context**: Email address that conflicts
    #[error("User with email '{email}' already exists")]
    UserAlreadyExists { email: String },
    
    /// Invalid Input Error
    /// 
    /// **When**: General input validation failures
    /// **HTTP Status**: 400 Bad Request
    /// **Context**: Description of what was invalid
    #[error("Invalid input: {message}")]
    InvalidInput { message: String },
    
    /// Database Error
    /// 
    /// **When**: Database operations fail
    /// **HTTP Status**: 500 Internal Server Error
    /// **Context**: Database error details (for logging)
    #[error("Database error: {message}")]
    DatabaseError { message: String },
    
    /// Internal Server Error
    /// 
    /// **When**: Unexpected application errors
    /// **HTTP Status**: 500 Internal Server Error
    /// **Context**: Error details for debugging
    #[error("Internal server error: {message}")]
    InternalError { message: String },
    
    /// Validation Error
    /// 
    /// **When**: Field-specific validation failures
    /// **HTTP Status**: 400 Bad Request
    /// **Context**: Field name and specific validation message
    #[error("Validation error: {field} - {message}")]
    ValidationError { field: String, message: String },
}

/// HTTP Response Error Implementation
/// 
/// This implementation of `ResponseError` converts our domain errors
/// into HTTP responses that Actix-Web can return to clients.
/// 
/// ## ResponseError Trait:
/// - Required for Actix-Web error handling
/// - Provides automatic conversion from errors to HTTP responses
/// - Enables using `?` operator in handlers
/// - Integrates with Actix-Web's error handling middleware
/// 
/// ## Response Format:
/// All errors return JSON with consistent structure:
/// ```json
/// {
///   "error": "error_type",
///   "message": "User-friendly message",
///   "code": 400
/// }
/// ```
/// 
/// ## Error Mapping Strategy:
/// - **4xx errors**: Client errors (validation, not found, etc.)
/// - **5xx errors**: Server errors (database, internal, etc.)
/// - **Consistent structure**: Same JSON format for all errors
/// - **Security**: Don't expose sensitive internal details
impl ResponseError for AppError {
    fn error_response(&self) -> HttpResponse {
        match self {
            // 404 Not Found - User doesn't exist
            AppError::UserNotFound { id } => HttpResponse::NotFound().json(json!({
                "error": "not_found",
                "message": format!("User with ID {} not found", id),
                "code": 404
            })),
            
            // 409 Conflict - User already exists
            AppError::UserAlreadyExists { email } => HttpResponse::Conflict().json(json!({
                "error": "conflict",
                "message": format!("User with email '{}' already exists", email),
                "code": 409
            })),
            
            // 400 Bad Request - Invalid input
            AppError::InvalidInput { message } => HttpResponse::BadRequest().json(json!({
                "error": "invalid_input",
                "message": message,
                "code": 400
            })),
            
            // 400 Bad Request - Field validation error
            AppError::ValidationError { field, message } => HttpResponse::BadRequest().json(json!({
                "error": "validation_error",
                "message": format!("Validation failed for field '{}': {}", field, message),
                "code": 400
            })),
            
            // 500 Internal Server Error - Database error
            AppError::DatabaseError { message } => HttpResponse::InternalServerError().json(json!({
                "error": "database_error",
                "message": "Database operation failed",
                "details": message,  // Could be omitted in production for security
                "code": 500
            })),
            
            // 500 Internal Server Error - General internal error
            AppError::InternalError { message } => HttpResponse::InternalServerError().json(json!({
                "error": "internal_error",
                "message": "Internal server error",
                "details": message,  // Could be omitted in production for security
                "code": 500
            })),
        }
    }
}

/// Application Result Type Alias
/// 
/// This type alias provides a convenient way to return results throughout
/// the application with our custom error type.
/// 
/// ## Type Alias Benefits:
/// - **Conciseness**: `AppResult<T>` instead of `Result<T, AppError>`
/// - **Consistency**: Same error type used everywhere
/// - **Maintainability**: Change error type in one place
/// - **Clarity**: Clear that this is an application-level result
/// 
/// ## Usage Pattern:
/// ```rust
/// async fn create_user(dto: CreateUserDto) -> AppResult<UserResponseDto> {
///     // ... implementation
/// }
/// ```
pub type AppResult<T> = Result<T, AppError>;

/// Helper function to create validation errors
/// 
/// This function provides a convenient way to create field-specific
/// validation errors with consistent formatting.
/// 
/// ## Usage:
/// ```rust
/// return Err(validation_error("email", "Invalid email format"));
/// ```
/// 
/// ## Benefits:
/// - **Consistency**: All validation errors have the same structure
/// - **Convenience**: No need to construct the error manually
/// - **Maintainability**: Change validation error format in one place
pub fn validation_error(field: &str, message: &str) -> AppError {
    AppError::ValidationError {
        field: field.to_string(),
        message: message.to_string(),
    }
}

/// Helper function to create invalid input errors
/// 
/// This function provides a convenient way to create general
/// input validation errors.
/// 
/// ## Usage:
/// ```rust
/// return Err(invalid_input("At least one field must be provided"));
/// ```
/// 
/// ## When to use:
/// - General input validation that doesn't relate to a specific field
/// - Business rule violations
/// - Format or structure errors
pub fn invalid_input(message: &str) -> AppError {
    AppError::InvalidInput {
        message: message.to_string(),
    }
}

/// Helper function to create internal errors
/// 
/// This function provides a convenient way to create internal
/// server errors for unexpected conditions.
/// 
/// ## Usage:
/// ```rust
/// return Err(internal_error("Unexpected state in user validation"));
/// ```
/// 
/// ## When to use:
/// - Unexpected application states
/// - Programming errors that shouldn't happen
/// - Fallback error handling
/// 
/// ## Security Note:
/// In production, be careful about exposing internal error details
/// to clients. Consider logging detailed errors and returning
/// generic messages to clients.
pub fn internal_error(message: &str) -> AppError {
    AppError::InternalError {
        message: message.to_string(),
    }
}
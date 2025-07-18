//! # HTTP Handlers Module
//! 
//! This module contains the **HTTP handling layer** of our Clean Architecture.
//! Handlers are responsible for:
//! 
//! 1. **HTTP-specific concerns**: Request/response formatting, status codes
//! 2. **Input validation**: Extracting and validating request data
//! 3. **Delegation**: Calling the appropriate service methods
//! 4. **Response formatting**: Converting service results to HTTP responses
//! 
//! ## Clean Architecture Position:
//! ```
//! HTTP Request → Routes → **[HANDLERS]** → Services → Repositories → Database
//! ```
//! 
//! ## Key Actix-Web Handler Concepts:
//! - **Extractors**: `web::Json`, `web::Path`, `web::Data` extract request data
//! - **Dependency Injection**: Services are injected via `web::Data`
//! - **Error Handling**: Custom errors are converted to HTTP responses
//! - **Async Handlers**: All handlers are async functions

use crate::models::{CreateUserDto, UpdateUserDto};
use crate::services::UserService;
use actix_web::{web, HttpResponse, ResponseError, Result};
use serde_json::json;
use std::sync::Arc;
use uuid::Uuid;

/// User Handler Structure
/// 
/// This struct represents a collection of HTTP handlers for user operations.
/// In this implementation, we use static methods instead of instance methods
/// for handlers, which is the common pattern in Actix-Web.
/// 
/// ## Note on Design:
/// The `service` field is not used in the current implementation because
/// we're using dependency injection via `web::Data` directly in handlers.
/// This demonstrates two different approaches to handler organization.
pub struct UserHandler {
    service: Arc<dyn UserService>,
}

impl UserHandler {
    /// Creates a new UserHandler instance
    /// 
    /// This constructor is provided for completeness but isn't used in the
    /// current implementation since we're using static handler methods.
    pub fn new(service: Arc<dyn UserService>) -> Self {
        Self { service }
    }
    
    /// Create User Handler
    /// 
    /// **HTTP Method**: POST /users
    /// **Purpose**: Creates a new user in the system
    /// 
    /// ## Actix-Web Extractors Demonstrated:
    /// - `web::Data<Arc<dyn UserService>>`: Extracts shared application state
    /// - `web::Json<CreateUserDto>`: Extracts and deserializes JSON request body
    /// 
    /// ## Error Handling Pattern:
    /// - Service errors are converted to HTTP responses using `ResponseError` trait
    /// - Success responses follow a consistent JSON structure
    /// 
    /// ## HTTP Status Codes:
    /// - `201 Created`: User successfully created
    /// - `400 Bad Request`: Invalid input data
    /// - `409 Conflict`: User already exists
    pub async fn create_user(
        // Extract the user service from application state
        // web::Data provides thread-safe access to shared state
        data: web::Data<Arc<dyn UserService>>,
        // Extract and validate JSON payload from request body
        // Actix-Web automatically deserializes JSON to CreateUserDto
        payload: web::Json<CreateUserDto>,
    ) -> Result<HttpResponse> {
        // Call the service layer to create the user
        // payload.into_inner() extracts the DTO from the Json wrapper
        match data.create_user(payload.into_inner()).await {
            Ok(user) => {
                // Return success response with 201 Created status
                Ok(HttpResponse::Created().json(json!({
                    "success": true,
                    "data": user
                })))
            }
            Err(e) => {
                // Convert service error to HTTP response
                // The ResponseError trait handles the conversion
                Ok(e.error_response())
            }
        }
    }
    
    /// Get User by ID Handler
    /// 
    /// **HTTP Method**: GET /users/{id}
    /// **Purpose**: Retrieves a specific user by their ID
    /// 
    /// ## Path Parameter Extraction:
    /// - `web::Path<Uuid>`: Extracts the `{id}` parameter from the URL
    /// - Actix-Web automatically validates and parses the UUID
    /// - Returns 400 Bad Request if the UUID format is invalid
    pub async fn get_user_by_id(
        // Extract the user service from application state
        data: web::Data<Arc<dyn UserService>>,
        // Extract the user ID from the URL path
        // This corresponds to the {id} parameter in the route
        path: web::Path<Uuid>,
    ) -> Result<HttpResponse> {
        // Extract the UUID from the path extractor
        let user_id = path.into_inner();
        
        // Call the service to retrieve the user
        match data.get_user_by_id(user_id).await {
            Ok(user) => {
                // Return the user data with 200 OK status
                Ok(HttpResponse::Ok().json(json!({
                    "success": true,
                    "data": user
                })))
            }
            Err(e) => {
                // Handle errors (e.g., user not found -> 404)
                Ok(e.error_response())
            }
        }
    }
    
    /// Get All Users Handler
    /// 
    /// **HTTP Method**: GET /users
    /// **Purpose**: Retrieves a list of all users
    /// 
    /// ## Simple Handler Pattern:
    /// This handler only needs the service dependency, no request data extraction
    pub async fn get_all_users(
        // Only need the service dependency for this handler
        data: web::Data<Arc<dyn UserService>>,
    ) -> Result<HttpResponse> {
        // Call the service to get all users
        match data.get_all_users().await {
            Ok(users_list) => {
                // Return the users list with pagination info
                Ok(HttpResponse::Ok().json(json!({
                    "success": true,
                    "data": users_list
                })))
            }
            Err(e) => {
                // Handle any service errors
                Ok(e.error_response())
            }
        }
    }
    
    /// Update User Handler
    /// 
    /// **HTTP Method**: PUT /users/{id}
    /// **Purpose**: Updates an existing user's information
    /// 
    /// ## Multiple Extractors:
    /// This handler demonstrates using multiple extractors:
    /// - Path parameter for the user ID
    /// - JSON body for the update data
    pub async fn update_user(
        // Extract the user service
        data: web::Data<Arc<dyn UserService>>,
        // Extract the user ID from the URL path
        path: web::Path<Uuid>,
        // Extract the update data from JSON body
        payload: web::Json<UpdateUserDto>,
    ) -> Result<HttpResponse> {
        // Extract the user ID from the path
        let user_id = path.into_inner();
        
        // Call the service to update the user
        match data.update_user(user_id, payload.into_inner()).await {
            Ok(user) => {
                // Return the updated user data
                Ok(HttpResponse::Ok().json(json!({
                    "success": true,
                    "data": user
                })))
            }
            Err(e) => {
                // Handle errors (not found, validation, etc.)
                Ok(e.error_response())
            }
        }
    }
    
    /// Delete User Handler
    /// 
    /// **HTTP Method**: DELETE /users/{id}
    /// **Purpose**: Deletes a user from the system
    /// 
    /// ## Delete Operation Pattern:
    /// - Success returns a confirmation message (no data)
    /// - Uses 200 OK status (could also use 204 No Content)
    pub async fn delete_user(
        // Extract the user service
        data: web::Data<Arc<dyn UserService>>,
        // Extract the user ID to delete
        path: web::Path<Uuid>,
    ) -> Result<HttpResponse> {
        // Extract the user ID from the path
        let user_id = path.into_inner();
        
        // Call the service to delete the user
        match data.delete_user(user_id).await {
            Ok(()) => {
                // Return success confirmation
                // Note: service returns () for successful deletion
                Ok(HttpResponse::Ok().json(json!({
                    "success": true,
                    "message": "User deleted successfully"
                })))
            }
            Err(e) => {
                // Handle errors (user not found, etc.)
                Ok(e.error_response())
            }
        }
    }
}
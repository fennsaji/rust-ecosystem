//! # Routes Configuration Module
//! 
//! This module defines the **routing layer** of our Clean Architecture.
//! It maps HTTP endpoints to handler functions and organizes routes logically.
//! 
//! ## Key Actix-Web Routing Concepts:
//! - **ServiceConfig**: Configuration builder for routes and services
//! - **web::scope()**: Groups related routes under a common path prefix
//! - **web::route()**: Maps HTTP methods to handler functions
//! - **Route Parameters**: Extract values from URL paths (e.g., `{id}`)
//! 
//! ## RESTful API Design:
//! This module demonstrates REST conventions:
//! - `POST /users` - Create resource
//! - `GET /users` - List resources
//! - `GET /users/{id}` - Get specific resource
//! - `PUT /users/{id}` - Update resource
//! - `DELETE /users/{id}` - Delete resource

use crate::handlers::UserHandler;
use actix_web::{web, HttpResponse, Result};
use serde_json::json;

/// Configure User-Related Routes
/// 
/// This function demonstrates **route organization** in Actix-Web.
/// It groups all user-related endpoints under the `/users` path.
/// 
/// ## Actix-Web Route Configuration:
/// - `web::scope()`: Creates a route group with a common prefix
/// - `web::post()`, `web::get()`, etc.: HTTP method matchers
/// - `web::to()`: Connects routes to handler functions
/// 
/// ## Route Parameters:
/// - `{id}` in the path becomes a parameter that handlers can extract
/// - Actix-Web automatically validates and parses these parameters
pub fn configure_user_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        // Create a route scope for all user-related endpoints
        // This prefixes all routes with "/users"
        web::scope("/users")
            // POST /users - Create a new user
            .route("", web::post().to(UserHandler::create_user))
            // GET /users - List all users
            .route("", web::get().to(UserHandler::get_all_users))
            // GET /users/{id} - Get a specific user by ID
            .route("/{id}", web::get().to(UserHandler::get_user_by_id))
            // PUT /users/{id} - Update a user
            .route("/{id}", web::put().to(UserHandler::update_user))
            // DELETE /users/{id} - Delete a user
            .route("/{id}", web::delete().to(UserHandler::delete_user)),
    );
}

/// Health Check Endpoint
/// 
/// A simple health check endpoint that returns server status.
/// This is commonly used by load balancers and monitoring systems.
/// 
/// ## Actix-Web Handler Pattern:
/// - `async fn` - All handlers must be async functions
/// - `Result<HttpResponse>` - Standard return type for handlers
/// - `HttpResponse::Ok()` - Builder pattern for HTTP responses
/// - `.json()` - Serializes data to JSON and sets content-type header
pub async fn health_check() -> Result<HttpResponse> {
    // Return a JSON response with server status
    Ok(HttpResponse::Ok().json(json!({
        "status": "healthy",
        "service": "actix-web-api",
        "version": "0.1.0"
    })))
}

/// Configure All Application Routes
/// 
/// This is the main route configuration function called from main.rs.
/// It demonstrates **modular route organization** by combining different
/// route groups into a single configuration.
/// 
/// ## Configuration Pattern:
/// - Single entry point for all routes
/// - Modular organization (health, users, etc.)
/// - Easy to extend with new route groups
pub fn configure_routes(cfg: &mut web::ServiceConfig) {
    cfg
        // Add health check endpoint
        .route("/health", web::get().to(health_check))
        // Add all user-related routes
        .configure(configure_user_routes);
}

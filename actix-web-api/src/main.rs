//! # Application Entry Point
//! 
//! This is the main module that bootstraps the entire Actix-Web application.
//! It demonstrates the **Clean Architecture** pattern by:
//! 
//! 1. **Dependency Injection**: All dependencies are created and wired here
//! 2. **Separation of Concerns**: Each layer (DB, Repository, Service, Handler) is independent
//! 3. **Configuration**: Server setup, middleware, and routing configuration
//! 
//! ## Clean Architecture Flow:
//! ```
//! HTTP Request → Routes → Handlers → Services → Repositories → Database
//! ```
//! 
//! ## Key Actix-Web Concepts Demonstrated:
//! - `HttpServer`: The main server that handles HTTP requests
//! - `App`: Application factory that creates app instances per worker
//! - `web::Data`: Shared application state (dependency injection)
//! - Middleware: Cross-cutting concerns like logging and tracing

// Module declarations - these make the modules available to this crate
mod db;         // Database connection management
mod entities;   // SeaORM entity models
mod errors;     // Custom error types and HTTP error responses
mod handlers;   // HTTP request handlers (controllers in MVC terms)
mod models;     // Domain models and DTOs
mod repositories; // Data access layer abstractions
mod routes;     // Route definitions and configuration
mod services;   // Business logic layer
mod utils;      // Shared utilities and helpers

// Actix-Web core imports
use actix_web::{middleware::Logger, web, App, HttpServer};
// Our application layers
use repositories::{PostgresUserRepository, UserRepository};
use routes::configure_routes;
use services::{UserService, UserServiceImpl};
// Standard library for shared ownership across threads
use std::sync::Arc;
// Tracing middleware for request logging
use tracing_actix_web::TracingLogger;

/// Dependency Injection Container
/// 
/// This function demonstrates the **Dependency Injection** pattern in Rust.
/// It creates and wires all dependencies in the correct order, following
/// the dependency flow: Database → Repository → Service
/// 
/// ## Why Arc<dyn Trait>?
/// - `Arc`: Allows shared ownership across multiple threads (Actix workers)
/// - `dyn Trait`: Enables runtime polymorphism (we can swap implementations)
/// - This pattern makes testing easier (we can inject mock implementations)
/// 
/// ## Error Handling Pattern:
/// Database errors are converted to IO errors for the main function
async fn setup_dependencies() -> std::io::Result<Arc<dyn UserService>> {
    // Initialize database connection pool
    // This creates a connection pool that can be shared across all requests
    let db_connection = db::init_db().await.map_err(|e| {
        // Convert database errors to IO errors for main function compatibility
        std::io::Error::new(std::io::ErrorKind::Other, format!("Database connection failed: {}", e))
    })?;
    
    // Create repository layer with PostgreSQL implementation
    // Arc<dyn Trait> allows us to use trait objects for dependency injection
    let user_repository: Arc<dyn UserRepository> = Arc::new(PostgresUserRepository::new(db_connection));
    
    // Create service layer with injected repository
    // The service layer doesn't know about the database - it only knows about the repository trait
    Ok(Arc::new(UserServiceImpl::new(user_repository)))
}

/// Application Entry Point
/// 
/// The `#[actix_web::main]` macro sets up the Tokio async runtime
/// and provides the main entry point for the Actix-Web application.
/// 
/// ## Actix-Web Server Architecture:
/// 1. **HttpServer**: Creates and manages worker threads
/// 2. **App Factory**: Creates new App instances for each worker
/// 3. **Shared State**: `web::Data` is shared across all workers
/// 4. **Middleware**: Applied to all requests in the order they're added
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Initialize structured logging
    // This sets up tracing for the entire application
    tracing_subscriber::fmt::init();
    
    // Setup dependency injection
    // This creates all our services and repositories
    let user_service = setup_dependencies().await?;
    
    tracing::info!("Starting Actix-Web API server on http://localhost:8080");
    
    // Create and start the HTTP server
    HttpServer::new(move || {
        // App factory function - called once per worker thread
        // Each worker gets its own App instance but shares the same data
        App::new()
            // Inject shared application state
            // web::Data wraps our service in application-managed state
            // This allows handlers to access the service via dependency injection
            .app_data(web::Data::new(user_service.clone()))
            // Configure all routes
            // This calls our route configuration function
            .configure(configure_routes)
            // Add middleware (applied in reverse order)
            // TracingLogger provides detailed request tracing
            .wrap(TracingLogger::default())
            // Logger provides basic request logging
            .wrap(Logger::default())
    })
    // Bind to localhost:8080
    .bind("127.0.0.1:8080")?
    // Start the server (this blocks until shutdown)
    .run()
    .await
}
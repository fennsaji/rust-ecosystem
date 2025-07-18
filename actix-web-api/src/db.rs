//! # Database Connection Management
//! 
//! This module handles **database connectivity** and **connection pooling** for our application.
//! It's responsible for:
//! 
//! 1. **Connection Setup**: Establishing database connections using SeaORM
//! 2. **Environment Configuration**: Reading database URL from environment variables
//! 3. **Connection Pooling**: Managing database connections efficiently
//! 4. **Error Handling**: Providing proper error handling for database operations
//! 
//! ## Clean Architecture Position:
//! ```
//! HTTP Request → Routes → Handlers → Services → Repositories → **[DATABASE]**
//! ```
//! 
//! ## Key Database Patterns:
//! - **Connection Pooling**: SeaORM automatically manages connection pools
//! - **Environment Configuration**: Database URL from .env file
//! - **Async Operations**: All database operations are asynchronous
//! - **Error Propagation**: Database errors are properly handled and propagated

use sea_orm::{Database, DatabaseConnection, DbErr};
use std::env;
use tracing::info;

/// Database Manager Structure
/// 
/// This struct manages the database connection for our application.
/// It wraps SeaORM's `DatabaseConnection` and provides a clean interface
/// for database operations.
/// 
/// ## SeaORM Connection Pattern:
/// - `DatabaseConnection` is clone-able and thread-safe
/// - It internally manages a connection pool
/// - Each clone shares the same underlying pool
/// - Connections are automatically returned to the pool when dropped
pub struct DatabaseManager {
    // SeaORM database connection (includes connection pooling)
    connection: DatabaseConnection,
}

impl DatabaseManager {
    /// Creates a new database manager with connection pool
    /// 
    /// This function demonstrates the **database initialization pattern**:
    /// 1. Read configuration from environment
    /// 2. Establish connection with automatic pooling
    /// 3. Verify connection is working
    /// 4. Return managed connection
    /// 
    /// ## Environment Configuration:
    /// - Reads `DATABASE_URL` from environment variables
    /// - Format: `postgres://user:password@host:port/database`
    /// - Can be set via `.env` file or system environment
    /// 
    /// ## Connection Pooling:
    /// SeaORM automatically creates a connection pool with:
    /// - Multiple connections for concurrent operations
    /// - Connection reuse for efficiency
    /// - Automatic connection health checks
    /// - Configurable pool size and timeouts
    pub async fn new() -> Result<Self, DbErr> {
        // Read database URL from environment
        // This will panic if DATABASE_URL is not set (fail-fast principle)
        let database_url = env::var("DATABASE_URL")
            .expect("DATABASE_URL environment variable must be set");
        
        info!("Connecting to database: {}", database_url);
        
        // Connect to database with automatic connection pooling
        // SeaORM creates a connection pool behind the scenes
        let connection = Database::connect(&database_url).await?;
        
        info!("Database connection established successfully");
        
        Ok(Self { connection })
    }
    
    /// Get a reference to the database connection
    /// 
    /// This method provides access to the underlying database connection.
    /// The connection is thread-safe and can be shared across operations.
    pub fn get_connection(&self) -> &DatabaseConnection {
        &self.connection
    }
    
    /// Get a cloned database connection
    /// 
    /// This method provides an owned copy of the database connection.
    /// 
    /// ## Connection Cloning Pattern:
    /// - Cloning a `DatabaseConnection` is cheap (just clones the pool handle)
    /// - All clones share the same underlying connection pool
    /// - This allows passing connections to different parts of the application
    /// - Each clone can be used independently but shares the same pool
    pub fn get_connection_owned(&self) -> DatabaseConnection {
        self.connection.clone()
    }
}

/// Initialize Database Connection Pool
/// 
/// This is the main entry point for database initialization.
/// It demonstrates the **initialization pattern** used throughout the application:
/// 1. Load environment variables
/// 2. Create database manager
/// 3. Extract and return the connection
/// 
/// ## Environment Loading:
/// - `dotenvy::dotenv()` loads variables from `.env` file
/// - `.ok()` means we don't fail if `.env` file doesn't exist
/// - This allows deployment flexibility (env file vs system env vars)
/// 
/// ## Error Handling:
/// - Returns `DbErr` if connection fails
/// - Caller is responsible for handling connection errors
/// - In main.rs, this is converted to IO error for consistency
pub async fn init_db() -> Result<DatabaseConnection, DbErr> {
    // Load environment variables from .env file (if present)
    // This is safe to call multiple times
    dotenvy::dotenv().ok();
    
    // Create database manager with connection pool
    let database_manager = DatabaseManager::new().await?;
    
    // Return the connection for use throughout the application
    Ok(database_manager.get_connection_owned())
}
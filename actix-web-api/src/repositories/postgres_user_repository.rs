use crate::entities::user::{self, Entity as UserEntity};
use crate::errors::{AppError, AppResult};
use crate::models::{CreateUserDto, UpdateUserDto, User};
use crate::repositories::UserRepository;
use async_trait::async_trait;
use sea_orm::*;
use uuid::Uuid;

/// PostgreSQL implementation of UserRepository using SeaORM
pub struct PostgresUserRepository {
    db: DatabaseConnection,
}

impl PostgresUserRepository {
    pub fn new(db: DatabaseConnection) -> Self {
        Self { db }
    }
}

#[async_trait]
impl UserRepository for PostgresUserRepository {
    async fn create(&self, create_dto: CreateUserDto) -> AppResult<User> {
        // Check if user with email already exists
        if self.exists_by_email(&create_dto.email).await? {
            return Err(AppError::UserAlreadyExists {
                email: create_dto.email,
            });
        }
        
        let user = User::new(create_dto.email, create_dto.name);
        let active_model = user::ActiveModel::from(user.clone());
        
        let _inserted = UserEntity::insert(active_model)
            .exec(&self.db)
            .await
            .map_err(|e| AppError::DatabaseError {
                message: e.to_string(),
            })?;
        
        Ok(user)
    }
    
    async fn find_by_id(&self, id: Uuid) -> AppResult<Option<User>> {
        let user = UserEntity::find_by_id(id)
            .one(&self.db)
            .await
            .map_err(|e| AppError::DatabaseError {
                message: e.to_string(),
            })?;
        
        Ok(user.map(User::from))
    }
    
    async fn find_by_email(&self, email: &str) -> AppResult<Option<User>> {
        let user = UserEntity::find()
            .filter(user::Column::Email.eq(email))
            .one(&self.db)
            .await
            .map_err(|e| AppError::DatabaseError {
                message: e.to_string(),
            })?;
        
        Ok(user.map(User::from))
    }
    
    async fn find_all(&self) -> AppResult<Vec<User>> {
        let users = UserEntity::find()
            .all(&self.db)
            .await
            .map_err(|e| AppError::DatabaseError {
                message: e.to_string(),
            })?;
        
        Ok(users.into_iter().map(User::from).collect())
    }
    
    async fn update(&self, id: Uuid, update_dto: UpdateUserDto) -> AppResult<User> {
        // Check if email is being updated and if it conflicts with existing user
        if let Some(ref new_email) = update_dto.email {
            let existing_user = UserEntity::find()
                .filter(user::Column::Email.eq(new_email))
                .filter(user::Column::Id.ne(id))
                .one(&self.db)
                .await
                .map_err(|e| AppError::DatabaseError {
                    message: e.to_string(),
                })?;
            
            if existing_user.is_some() {
                return Err(AppError::UserAlreadyExists {
                    email: new_email.clone(),
                });
            }
        }
        
        // Find the user to update
        let user = UserEntity::find_by_id(id)
            .one(&self.db)
            .await
            .map_err(|e| AppError::DatabaseError {
                message: e.to_string(),
            })?
            .ok_or(AppError::UserNotFound { id })?;
        
        // Convert to domain model and update
        let mut domain_user = User::from(user);
        domain_user.update(update_dto);
        
        // Convert back to ActiveModel and update
        let mut active_model: user::ActiveModel = domain_user.clone().into();
        active_model.id = Unchanged(id);
        
        let _updated = UserEntity::update(active_model)
            .exec(&self.db)
            .await
            .map_err(|e| AppError::DatabaseError {
                message: e.to_string(),
            })?;
        
        Ok(domain_user)
    }
    
    async fn delete(&self, id: Uuid) -> AppResult<()> {
        let delete_result = UserEntity::delete_by_id(id)
            .exec(&self.db)
            .await
            .map_err(|e| AppError::DatabaseError {
                message: e.to_string(),
            })?;
        
        if delete_result.rows_affected == 0 {
            return Err(AppError::UserNotFound { id });
        }
        
        Ok(())
    }
    
    async fn exists_by_email(&self, email: &str) -> AppResult<bool> {
        let count = UserEntity::find()
            .filter(user::Column::Email.eq(email))
            .count(&self.db)
            .await
            .map_err(|e| AppError::DatabaseError {
                message: e.to_string(),
            })?;
        
        Ok(count > 0)
    }
}